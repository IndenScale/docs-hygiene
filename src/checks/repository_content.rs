fn check_max_lines(
    root: &Path,
    config: &Config,
    docs: &[DocFile],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    let base_limits = normalized_bases(config)
        .into_iter()
        .map(|base| (base.id, base.max_lines))
        .collect::<BTreeMap<_, _>>();
    for doc in docs {
        let Some(max_lines) = base_limits.get(&doc.base_id).and_then(|value| *value) else {
            continue;
        };
        let text = std::fs::read_to_string(root.join(&doc.rel))?;
        let lines = text.lines().count();
        if lines > max_lines {
            diagnostics.push(Diagnostic::new(
                "DH_SIZE_001",
                Severity::Warning,
                doc.rel.display().to_string(),
                format!("Document has {lines} lines, exceeding maxLines {max_lines}."),
            ));
        }
    }
    Ok(())
}

fn check_ascii_art(
    root: &Path,
    config: &Config,
    docs: &[DocFile],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    if !config.docs.forbid_ascii_art {
        return Ok(());
    }

    for doc in docs {
        let text = std::fs::read_to_string(root.join(&doc.rel))?;
        let surface = ascii_art_surface(&text);
        let lines = surface.lines().collect::<Vec<_>>();
        let mut index = 0;
        while index < lines.len() {
            if !is_ascii_art_line(lines[index]) {
                index += 1;
                continue;
            }

            let start = index;
            let mut end = index + 1;
            while end < lines.len() && is_ascii_art_line(lines[end]) {
                end += 1;
            }
            let block = &lines[start..end];
            if block.len() >= 2 && block.iter().any(|line| is_strong_ascii_art_line(line)) {
                diagnostics.push(
                    Diagnostic::new(
                        "DH_ASCII_001",
                        Severity::Error,
                        doc.rel.display().to_string(),
                        "ASCII art is forbidden in documentation; use Markdown structure or an image instead.",
                    )
                    .at_line(start + 1),
                );
            }
            index = end;
        }
    }

    Ok(())
}

fn is_ascii_art_line(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.len() < 3 {
        return false;
    }
    if trimmed.starts_with("- ") {
        return false;
    }

    // Do not treat ordinary Markdown tables as diagrams.
    let table_cells = trimmed
        .strip_prefix('|')
        .and_then(|value| value.strip_suffix('|'))
        .map(|value| {
            value
                .split('|')
                .filter(|cell| !cell.trim().is_empty())
                .count()
        })
        .unwrap_or(0);
    if table_cells >= 2 && !trimmed.contains("->") && !trimmed.contains("<-") {
        return false;
    }

    let art_count = trimmed
        .chars()
        .filter(|ch| "+-|=/_\\<>[]{}()#*.:".contains(*ch))
        .count();
    if art_count < 2 {
        return false;
    }

    let has_alphanumeric = trimmed.chars().any(|ch| ch.is_ascii_alphanumeric());
    if !has_alphanumeric {
        // Horizontal rules are Markdown, not ASCII art.
        return !trimmed.chars().all(|ch| matches!(ch, '-' | '*' | '_'));
    }

    trimmed.contains('|')
        || trimmed.contains("->")
        || trimmed.contains("<-")
        || trimmed
            .chars()
            .next()
            .is_some_and(|ch| matches!(ch, '+' | '[' | '(' | '/' | '\\'))
        || trimmed
            .chars()
            .next_back()
            .is_some_and(|ch| matches!(ch, '+' | ']' | ')' | '/' | '\\'))
}

fn is_strong_ascii_art_line(line: &str) -> bool {
    let trimmed = line.trim();
    !trimmed.chars().any(|ch| ch.is_ascii_alphanumeric())
        || trimmed.contains("->")
        || trimmed.contains("<-")
        || trimmed.starts_with('+')
        || trimmed.ends_with('+')
}

fn check_language(
    root: &Path,
    config: &Config,
    docs: &[DocFile],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    let Some(canonical_language) = config.language_representations.canonical.as_ref() else {
        return Ok(());
    };
    if config.language.is_empty() {
        return Ok(());
    }

    for doc in docs {
        let lang = doc.lang.as_ref().unwrap_or(canonical_language);
        let Some(rule) = config.language.get(lang) else {
            continue;
        };
        let text = strip_code_blocks(&std::fs::read_to_string(root.join(&doc.rel))?);
        let ratio = cjk_ratio(&text);
        if let Some(min) = rule.min_cjk_ratio {
            if ratio < min {
                diagnostics.push(Diagnostic::new(
                    "DH_LANG_001",
                    Severity::Warning,
                    doc.rel.display().to_string(),
                    format!(
                        "CJK ratio {:.3} is below configured minCjkRatio {:.3} for {lang}.",
                        ratio, min
                    ),
                ));
            }
        }
        if let Some(max) = rule.max_cjk_ratio {
            if ratio > max {
                diagnostics.push(Diagnostic::new(
                    "DH_LANG_002",
                    Severity::Warning,
                    doc.rel.display().to_string(),
                    format!(
                        "CJK ratio {:.3} is above configured maxCjkRatio {:.3} for {lang}.",
                        ratio, max
                    ),
                ));
            }
        }
    }

    Ok(())
}

fn check_concepts(
    root: &Path,
    config: &Config,
    docs: &[DocFile],
    ignore: &GlobSet,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    if !config.concepts.require_concept_file {
        return Ok(());
    }

    let concept_dir = root.join(&config.concepts.dir);
    let mut concept_names = BTreeSet::new();
    if concept_dir.exists() {
        for entry in WalkDir::new(&concept_dir) {
            let entry = entry?;
            if entry.file_type().is_file()
                && entry.path().extension().and_then(|value| value.to_str()) == Some("md")
            {
                let rel = entry.path().strip_prefix(root).unwrap_or(entry.path());
                if ignore.is_match(rel) {
                    continue;
                }
                if let Some(stem) = entry.path().file_stem().and_then(|value| value.to_str()) {
                    concept_names.insert(stem.to_string());
                }
            }
        }
    }

    let bold = Regex::new(r"\*\*(?P<term>[^*\n][^*\n]{1,80})\*\*")?;
    let mut referenced = BTreeSet::new();
    for doc in docs {
        let text = strip_code_blocks(&std::fs::read_to_string(root.join(&doc.rel))?);
        for (idx, line) in text.lines().enumerate() {
            for captures in bold.captures_iter(line) {
                let term = captures["term"].trim();
                if is_probable_concept(term) {
                    referenced.insert(term.to_string());
                    if !concept_names.contains(term) {
                        diagnostics.push(
                            Diagnostic::new(
                                "DH_CONCEPT_001",
                                Severity::Error,
                                doc.rel.display().to_string(),
                                format!(
                                    "Term \"{term}\" is not defined in {}/{}.md.",
                                    config.concepts.dir.display(),
                                    term
                                ),
                            )
                            .at_line(idx + 1),
                        );
                    }
                }
            }
        }
    }

    if config.concepts.fail_on_orphan_concept.as_deref() == Some("warn") {
        for concept in concept_names.difference(&referenced) {
            diagnostics.push(Diagnostic::new(
                "DH_CONCEPT_002",
                Severity::Warning,
                format!("{}/{}.md", config.concepts.dir.display(), concept),
                "Concept file is not referenced by docs.",
            ));
        }
    }

    Ok(())
}
