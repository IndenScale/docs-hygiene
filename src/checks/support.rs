fn check_adapters(root: &Path, config: &Config, diagnostics: &mut Vec<Diagnostic>) -> Result<()> {
    let markdownlint = &config.adapters.markdownlint;
    if !markdownlint.enabled {
        return Ok(());
    }

    let command = markdownlint
        .command
        .as_deref()
        .unwrap_or("markdownlint-cli2");
    let output = Command::new(command)
        .args(&markdownlint.args)
        .current_dir(root)
        .output();

    let output = match output {
        Ok(output) => output,
        Err(error) => {
            diagnostics.push(
                Diagnostic::new(
                    "DH_ADAPTER_001",
                    Severity::Error,
                    ".",
                    format!("Failed to run markdownlint adapter `{command}`: {error}."),
                )
                .with_source("markdownlint"),
            );
            return Ok(());
        }
    };

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let details = [stdout.trim(), stderr.trim()]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        diagnostics.push(
            Diagnostic::new(
                "DH_ADAPTER_001",
                Severity::Error,
                ".",
                format!("markdownlint adapter failed:\n{details}"),
            )
            .with_source("markdownlint"),
        );
    }

    Ok(())
}

fn apply_suppressions(
    config: &Config,
    diagnostics: Vec<Diagnostic>,
) -> Result<(Vec<Diagnostic>, Vec<SuppressedDiagnostic>)> {
    let suppressions = config
        .suppressions
        .iter()
        .map(|suppression| {
            let mut builder = GlobSetBuilder::new();
            for path in &suppression.paths {
                builder.add(Glob::new(path)?);
            }
            Ok((suppression, builder.build()?))
        })
        .collect::<Result<Vec<_>, globset::Error>>()?;

    let mut visible = Vec::new();
    let mut suppressed = Vec::new();
    for diagnostic in diagnostics {
        let matched = suppressions.iter().find(|(suppression, paths)| {
                let _reason = suppression.reason.as_deref();
                let code_matches = suppression.code == "*" || suppression.code == diagnostic.code;
                let path_matches =
                    suppression.paths.is_empty() || paths.is_match(Path::new(&diagnostic.path));
                code_matches && path_matches
        });
        if let Some((suppression, _)) = matched {
            suppressed.push(SuppressedDiagnostic {
                code: diagnostic.code.to_owned(),
                path: diagnostic.path,
                reason: suppression.reason.clone(),
            });
        } else {
            visible.push(diagnostic);
        }
    }
    Ok((visible, suppressed))
}

fn grouped_docs<'a>(docs: &[&'a DocFile]) -> BTreeMap<Option<String>, Vec<&'a DocFile>> {
    let mut groups = BTreeMap::<Option<String>, Vec<&DocFile>>::new();
    for doc in docs {
        groups.entry(doc.lang.clone()).or_default().push(doc);
    }
    groups
}

fn group_label(base_id: &str, lang: &Option<String>) -> String {
    match lang.as_deref() {
        Some(lang) => format!("{base_id}:{lang}"),
        None => base_id.to_string(),
    }
}

fn canonical_document_path(
    docs: &[DocFile],
    canonical_language: &str,
    key: &(u32, String),
) -> Option<String> {
    docs.iter()
        .find(|doc| {
            doc.numbered
                && doc.number == Some(key.0)
                && doc.stem == key.1
                && doc.lang.as_deref().unwrap_or(canonical_language) == canonical_language
        })
        .map(|doc| doc.rel.display().to_string())
}

fn localized_doc_path(docs: &[DocFile], lang: &str, key: &(u32, String)) -> Option<String> {
    docs.iter()
        .find(|doc| {
            doc.numbered
                && doc.number == Some(key.0)
                && doc.stem == key.1
                && doc.lang.as_deref() == Some(lang)
        })
        .map(|doc| doc.rel.display().to_string())
}

fn normalized_bases(config: &Config) -> Vec<NormalizedBase> {
    if !config.docs.bases.is_empty() {
        return config
            .docs
            .bases
            .iter()
            .map(|base| NormalizedBase {
                id: base.id.clone(),
                root: base.root.clone(),
                localized_roots: base.localized_roots.clone(),
                patterns: if base.patterns.is_empty() {
                    default_patterns(&config.docs.filename_pattern)
                } else {
                    base.patterns.clone()
                },
                require_continuous_numbering: base
                    .require_continuous_numbering
                    .unwrap_or(config.docs.require_continuous_numbering),
                max_lines: base.max_lines.or(config.docs.max_lines),
                ignore: base.ignore.clone(),
            })
            .collect();
    }

    vec![NormalizedBase {
        id: "default".to_string(),
        root: config.docs.root.clone(),
        localized_roots: BTreeMap::new(),
        patterns: default_patterns(&config.docs.filename_pattern),
        require_continuous_numbering: config.docs.require_continuous_numbering,
        max_lines: config.docs.max_lines,
        ignore: Vec::new(),
    }]
}

fn default_patterns(regex: &str) -> Vec<FilenamePatternConfig> {
    vec![FilenamePatternConfig {
        id: "numbered".to_string(),
        regex: regex.to_string(),
        document_kind: "numbered".to_string(),
        numbered: true,
    }]
}

fn compile_patterns(
    patterns: &[FilenamePatternConfig],
) -> Result<Vec<(Regex, &FilenamePatternConfig)>> {
    patterns
        .iter()
        .map(|pattern| Ok((Regex::new(&pattern.regex)?, pattern)))
        .collect()
}

fn matching_pattern<'a>(
    file_name: &str,
    patterns: &'a [(Regex, &'a FilenamePatternConfig)],
) -> Option<&'a FilenamePatternConfig> {
    patterns
        .iter()
        .find_map(|(regex, pattern)| regex.is_match(file_name).then_some(*pattern))
}

fn numbered_parts(file_name: &str) -> (Option<u32>, String) {
    let numbered = Regex::new(r"^(?<number>\d{2})_(?<stem>.+)\.md$").unwrap();
    if let Some(captures) = numbered.captures(file_name) {
        return (
            captures["number"].parse().ok(),
            captures["stem"].to_string(),
        );
    }
    (
        None,
        file_name
            .strip_suffix(".md")
            .unwrap_or(file_name)
            .to_string(),
    )
}

fn build_ignore(root: &Path, config: &Config) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    builder.add(Glob::new("target/**")?);
    builder.add(Glob::new(".git/**")?);
    for path in &config.ignore.paths {
        builder.add(Glob::new(path)?);
    }
    let _ = root;
    Ok(builder.build()?)
}

fn build_base_ignore(base: &NormalizedBase) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for path in &base.ignore {
        builder.add(Glob::new(path)?);
    }
    Ok(builder.build()?)
}

fn strip_code_blocks(text: &str) -> String {
    let mut in_code = false;
    let mut out = String::new();
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            in_code = !in_code;
            out.push('\n');
            continue;
        }
        if !in_code {
            out.push_str(line);
        }
        out.push('\n');
    }
    out
}

fn strip_markdown_code(text: &str) -> String {
    let without_fences = strip_code_blocks(text);
    let inline_code = Regex::new(r"`+[^`\n]*`+").expect("static inline code regex");
    inline_code.replace_all(&without_fences, "").into_owned()
}

fn ascii_art_surface(text: &str) -> String {
    let mut in_code = false;
    let mut include_code = false;
    let mut out = String::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        if let Some(info) = trimmed.strip_prefix("```") {
            if !in_code {
                let language = info
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_ascii_lowercase();
                include_code = matches!(
                    language.as_str(),
                    "text" | "txt" | "ascii" | "diagram" | "plaintext"
                );
            }
            in_code = !in_code;
            out.push('\n');
            continue;
        }
        if !in_code || include_code {
            out.push_str(line);
        }
        out.push('\n');
    }
    out
}

fn is_probable_concept(term: &str) -> bool {
    let trimmed = term.trim();
    if trimmed.len() < 2 || trimmed.len() > 80 {
        return false;
    }
    !trimmed.contains('/')
        && !trimmed.ends_with('.')
        && !trimmed.chars().all(|value| value.is_ascii_punctuation())
}

fn cjk_ratio(text: &str) -> f64 {
    let mut cjk = 0usize;
    let mut letters = 0usize;
    for ch in text.chars() {
        if ch.is_whitespace() || ch.is_ascii_punctuation() {
            continue;
        }
        if is_cjk(ch) {
            cjk += 1;
            letters += 1;
        } else if ch.is_alphabetic() {
            letters += 1;
        }
    }
    if letters == 0 {
        return 0.0;
    }
    cjk as f64 / letters as f64
}

fn is_cjk(ch: char) -> bool {
    matches!(
        ch as u32,
        0x4E00..=0x9FFF
            | 0x3400..=0x4DBF
            | 0x20000..=0x2A6DF
            | 0x2A700..=0x2B73F
            | 0x2B740..=0x2B81F
            | 0x2B820..=0x2CEAF
            | 0xF900..=0xFAFF
    )
}
