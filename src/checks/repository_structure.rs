fn check_required_files(root: &Path, config: &Config, diagnostics: &mut Vec<Diagnostic>) {
    let mut required = config.entry_docs.required.clone();
    required.extend(config.required_files.iter().cloned());
    for file in &required {
        if !root.join(file).exists() {
            diagnostics.push(Diagnostic::new(
                "DH_REQUIRED_001",
                Severity::Error,
                file.display().to_string(),
                "Required documentation file is missing.",
            ));
        }
    }
}

fn check_markdown_links(
    root: &Path,
    config: &Config,
    docs: &[DocFile],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    let inline_link = Regex::new(
        r#"!?(?:\[[^\]\n]*\])\((?:<([^>\n]+)>|([^\s)\n]+))(?:\s+[\"'][^)\n]*[\"'])?\)"#,
    )?;
    let reference_definition = Regex::new(r#"^\s{0,3}\[[^\]\n]+\]:\s*(?:<([^>\n]+)>|(\S+))"#)?;
    let uri_scheme = Regex::new(r"^[A-Za-z][A-Za-z0-9+.-]*:")?;
    let mut paths = docs
        .iter()
        .map(|doc| doc.rel.clone())
        .collect::<BTreeSet<_>>();
    paths.extend(
        config
            .entry_docs
            .required
            .iter()
            .chain(&config.entry_docs.optional)
            .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("md"))
            .filter(|path| root.join(path).is_file())
            .cloned(),
    );

    for rel in paths {
        let text = std::fs::read_to_string(root.join(&rel))?;
        let surface = strip_markdown_code(&text);
        let mut seen = BTreeSet::new();
        for (index, line) in surface.lines().enumerate() {
            let destinations = inline_link
                .captures_iter(line)
                .chain(reference_definition.captures_iter(line));
            for captures in destinations {
                let destination = captures
                    .get(1)
                    .or_else(|| captures.get(2))
                    .map(|value| value.as_str())
                    .unwrap_or("");
                if !seen.insert((index, destination.to_owned())) {
                    continue;
                }
                let Some(target) = resolve_repository_link(&rel, destination, &uri_scheme) else {
                    continue;
                };
                if target.as_os_str().is_empty() || !root.join(&target).exists() {
                    diagnostics.push(
                        Diagnostic::new(
                            "DH_LINK_001",
                            Severity::Error,
                            rel.display().to_string(),
                            format!(
                                "Markdown Link target '{destination}' does not resolve to a project-root path."
                            ),
                        )
                        .at_line(index + 1),
                    );
                }
            }
        }
    }
    Ok(())
}

fn resolve_repository_link(
    source: &Path,
    destination: &str,
    uri_scheme: &Regex,
) -> Option<PathBuf> {
    let destination = destination.trim();
    if destination.is_empty()
        || destination.starts_with('#')
        || destination.starts_with("//")
        || uri_scheme.is_match(destination)
    {
        return None;
    }
    let path = destination
        .split(['#', '?'])
        .next()
        .unwrap_or("")
        .replace("%20", " ");
    if path.is_empty() {
        return None;
    }

    let mut resolved = if path.starts_with('/') {
        PathBuf::new()
    } else {
        source
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf()
    };
    for component in Path::new(path.trim_start_matches('/')).components() {
        match component {
            Component::CurDir => {}
            Component::Normal(value) => resolved.push(value),
            Component::ParentDir => {
                if !resolved.pop() {
                    return Some(PathBuf::new());
                }
            }
            Component::RootDir | Component::Prefix(_) => return Some(PathBuf::new()),
        }
    }
    Some(resolved)
}

fn collect_docs(
    root: &Path,
    config: &Config,
    ignore: &GlobSet,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<Vec<DocFile>> {
    let bases = normalized_bases(config);
    let lang_set: BTreeSet<_> = config
        .language_representations
        .localized
        .iter()
        .cloned()
        .collect();
    let mut docs = Vec::new();

    for base in bases {
        let patterns = compile_patterns(&base.patterns)?;
        let base_ignore = build_base_ignore(&base)?;
        let roots = std::iter::once((base.root.clone(), None)).chain(
            base.localized_roots
                .iter()
                .map(|(lang, path)| (path.clone(), Some(lang.clone()))),
        );

        for (relative_root, explicit_lang) in roots {
            let docs_root = root.join(relative_root);
            if !docs_root.exists() {
                continue;
            }
            for entry in WalkDir::new(&docs_root) {
                let entry = entry?;
                if !entry.file_type().is_file() {
                    continue;
                }
                let path = entry.path();
                let rel = path.strip_prefix(root).unwrap_or(path);
                if ignore.is_match(rel)
                    || base_ignore.is_match(rel)
                    || path.extension().and_then(|value| value.to_str()) != Some("md")
                {
                    continue;
                }

                let file_name = path
                    .file_name()
                    .and_then(|value| value.to_str())
                    .unwrap_or("");
                let Some(pattern) = matching_pattern(file_name, &patterns) else {
                    diagnostics.push(Diagnostic::new(
                        "DH_NAME_001",
                        Severity::Error,
                        rel.display().to_string(),
                        format!("File name does not match any pattern for base {}.", base.id),
                    ));
                    continue;
                };

                let parent = path.parent().unwrap_or(&docs_root);
                let lang = explicit_lang.clone().or_else(|| {
                    parent
                        .strip_prefix(&docs_root)
                        .ok()
                        .and_then(|value| value.components().next())
                        .and_then(|value| value.as_os_str().to_str())
                        .filter(|value| lang_set.contains(*value))
                        .map(|value| value.to_string())
                });
                let (number, stem) = numbered_parts(file_name);
                docs.push(DocFile {
                    base_id: base.id.clone(),
                    rel: rel.to_path_buf(),
                    lang,
                    number,
                    stem,
                    numbered: pattern.numbered,
                });
            }
        }
    }

    Ok(docs)
}

fn check_numbering(config: &Config, docs: &[DocFile], diagnostics: &mut Vec<Diagnostic>) {
    for base in normalized_bases(config) {
        if !base.require_continuous_numbering {
            continue;
        }
        let base_docs = docs
            .iter()
            .filter(|doc| doc.base_id == base.id && doc.numbered)
            .collect::<Vec<_>>();
        for (lang, group) in grouped_docs(&base_docs) {
            let mut seen = BTreeMap::<u32, Vec<&DocFile>>::new();
            for doc in group {
                if let Some(number) = doc.number {
                    seen.entry(number).or_default().push(doc);
                }
            }

            for (number, files) in &seen {
                if files.len() > 1 {
                    let diagnostic = files.iter().fold(
                        Diagnostic::new(
                            "DH_SEQ_002",
                            Severity::Error,
                            group_label(&base.id, &lang),
                            format!("Duplicate document number {number:02}."),
                        ),
                        |diagnostic, file| {
                            diagnostic.with_related(RelatedInformation::new(
                                file.rel.display().to_string(),
                                format!("Uses duplicate number {number:02}."),
                            ))
                        },
                    );
                    diagnostics.push(diagnostic);
                }
            }

            let Some(max) = seen.keys().next_back().copied() else {
                continue;
            };
            for number in 1..=max {
                if !seen.contains_key(&number) {
                    diagnostics.push(Diagnostic::new(
                        "DH_SEQ_001",
                        Severity::Error,
                        group_label(&base.id, &lang),
                        format!("Missing document number {number:02}."),
                    ));
                }
            }
        }
    }
}

fn check_language_representations(
    config: &Config,
    docs: &[DocFile],
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !config.language_representations.require_document_parity
        && !config.language_representations.require_number_parity
    {
        return;
    }
    let Some(canonical_language) = config.language_representations.canonical.as_ref() else {
        return;
    };

    let mut by_lang = BTreeMap::<String, BTreeSet<(u32, String)>>::new();
    for doc in docs {
        if !doc.numbered {
            continue;
        }
        let lang = doc
            .lang
            .clone()
            .unwrap_or_else(|| canonical_language.clone());
        let Some(number) = doc.number else {
            continue;
        };
        by_lang
            .entry(lang)
            .or_default()
            .insert((number, doc.stem.clone()));
    }

    let canonical_documents = by_lang.get(canonical_language).cloned().unwrap_or_default();
    for lang in &config.language_representations.localized {
        let localized = by_lang.get(lang).cloned().unwrap_or_default();
        for key in &canonical_documents {
            if !localized.contains(key) {
                let canonical_path = canonical_document_path(docs, canonical_language, key);
                let mut diagnostic = Diagnostic::new(
                    "DH_REPRESENTATION_001",
                    Severity::Error,
                    lang.to_string(),
                    format!(
                        "Missing localized counterpart for {:02}_{}.md.",
                        key.0, key.1
                    ),
                );
                if let Some(path) = canonical_path {
                    diagnostic = diagnostic.with_related(RelatedInformation::new(
                        path,
                        "Canonical document that requires localization.",
                    ));
                }
                diagnostics.push(diagnostic);
            }
        }
        for key in &localized {
            if !canonical_documents.contains(key) {
                let path = localized_doc_path(docs, lang, key).unwrap_or_else(|| lang.to_string());
                diagnostics.push(Diagnostic::new(
                    "DH_REPRESENTATION_002",
                    Severity::Warning,
                    path,
                    format!(
                        "Localized document has no canonical counterpart: {:02}_{}.md.",
                        key.0, key.1
                    ),
                ));
            }
        }
    }
}
