fn check_document_contracts(
    root: &Path,
    config: &Config,
    ignore: &GlobSet,
    managed_docs: &[DocFile],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<DocumentTemplateReport> {
    let (profiles, template_report) = resolve_document_profiles(config, diagnostics);
    if profiles.is_empty()
        && config
            .document_contracts
            .maturity
            .recommendations
            .is_empty()
    {
        return Ok(template_report);
    }

    check_maturity_recommendation(root, config, ignore, managed_docs.len(), diagnostics)?;
    let declared = config.document_contracts.maturity.declared;

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| entry.depth() == 0 || entry.file_name() != ".git")
    {
        let entry = entry?;
        if !entry.file_type().is_file()
            || entry.path().extension().and_then(|value| value.to_str()) != Some("md")
        {
            continue;
        }
        let rel = entry.path().strip_prefix(root).unwrap_or(entry.path());
        if ignore.is_match(rel) {
            continue;
        }
        let Some(profile) = matching_document_profile(rel, &profiles)?
        else {
            continue;
        };
        check_document_contract(root, rel, profile, declared, diagnostics)?;
    }
    Ok(template_report)
}

fn matching_document_profile<'a>(
    rel: &Path,
    profiles: &'a [ResolvedDocumentProfile],
) -> Result<Option<&'a ResolvedDocumentProfile>> {
    let filename = rel
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    for profile in profiles {
        let matcher = &profile.matcher;
        if matcher.paths.is_empty() && matcher.filenames.is_empty() {
            continue;
        }
        let path_matches = if matcher.paths.is_empty() {
            true
        } else {
            let mut builder = GlobSetBuilder::new();
            for path in &matcher.paths {
                if let Ok(pattern) = Glob::new(path) {
                    builder.add(pattern);
                }
            }
            builder.build()?.is_match(rel)
        };
        let filename_matches = if matcher.filenames.is_empty() {
            true
        } else {
            matcher
                .filenames
                .iter()
                .filter_map(|pattern| Regex::new(pattern).ok())
                .any(|pattern| pattern.is_match(filename))
        };
        if path_matches && filename_matches {
            return Ok(Some(profile));
        }
    }
    Ok(None)
}

fn check_document_contract(
    root: &Path,
    rel: &Path,
    profile: &ResolvedDocumentProfile,
    declared: MaturityLevel,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    let text = std::fs::read_to_string(root.join(rel))?;
    let surface = strip_code_blocks(&text);
    let headings = markdown_headings(&surface);
    let severity = contract_severity(declared, profile.enforce_from);
    let mut matched_lines = Vec::new();

    for section in &profile.required_sections {
        let matched = headings.iter().find(|heading| {
            section
                .headings
                .iter()
                .any(|candidate| candidate == &heading.title)
        });
        let Some(heading) = matched else {
            diagnostics.push(Diagnostic::new(
                "DH_CONTRACT_001",
                severity,
                rel.display().to_string(),
                format!(
                    "Document profile '{}' requires section '{}' (accepted headings: {}).",
                    profile.id,
                    section.id,
                    section.headings.join(", ")
                ),
            ));
            continue;
        };
        matched_lines.push((section.id.as_str(), heading.line));

        if !profile.placeholder_patterns.is_empty()
            && section_contains_placeholder(
                &surface,
                &headings,
                heading.line,
                &profile.placeholder_patterns,
            )
        {
            let placeholder_severity = if declared > profile.placeholders_allowed_until {
                Severity::Error
            } else {
                Severity::Info
            };
            diagnostics.push(
                Diagnostic::new(
                    "DH_CONTRACT_003",
                    placeholder_severity,
                    rel.display().to_string(),
                    format!(
                        "Required section '{}' in profile '{}' still contains a declared placeholder.",
                        section.id, profile.id
                    ),
                )
                .at_line(heading.line),
            );
        }
    }

    if profile.ordered_sections && matched_lines.windows(2).any(|pair| pair[0].1 >= pair[1].1) {
        diagnostics.push(Diagnostic::new(
            "DH_CONTRACT_004",
            severity,
            rel.display().to_string(),
            format!(
                "Required sections for document profile '{}' are not in configured order.",
                profile.id
            ),
        ));
    }

    for field in &profile.required_fields {
        if Regex::new(&field.pattern).is_ok_and(|pattern| !pattern.is_match(&surface)) {
            diagnostics.push(Diagnostic::new(
                "DH_CONTRACT_002",
                severity,
                rel.display().to_string(),
                format!(
                    "Document profile '{}' requires field '{}'.",
                    profile.id, field.id
                ),
            ));
        }
    }
    Ok(())
}

#[derive(Debug)]
struct MarkdownHeading {
    line: usize,
    title: String,
}

fn markdown_headings(text: &str) -> Vec<MarkdownHeading> {
    text.lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let trimmed = line.trim_start();
            let hashes = trimmed.chars().take_while(|ch| *ch == '#').count();
            if !(1..=6).contains(&hashes) || !trimmed[hashes..].starts_with(' ') {
                return None;
            }
            let title = trimmed[hashes..]
                .trim()
                .trim_end_matches('#')
                .trim()
                .to_string();
            Some(MarkdownHeading {
                line: index + 1,
                title,
            })
        })
        .collect()
}

fn section_contains_placeholder(
    text: &str,
    headings: &[MarkdownHeading],
    heading_line: usize,
    patterns: &[String],
) -> bool {
    let end_line = headings
        .iter()
        .find(|heading| heading.line > heading_line)
        .map(|heading| heading.line)
        .unwrap_or_else(|| text.lines().count() + 1);
    let body = text
        .lines()
        .skip(heading_line)
        .take(end_line.saturating_sub(heading_line + 1))
        .collect::<Vec<_>>()
        .join("\n");
    for pattern in patterns {
        if Regex::new(pattern).is_ok_and(|pattern| pattern.is_match(&body)) {
            return true;
        }
    }
    false
}

fn contract_severity(declared: MaturityLevel, enforce_from: MaturityLevel) -> Severity {
    if declared >= enforce_from {
        Severity::Error
    } else {
        Severity::Warning
    }
}

fn check_maturity_recommendation(
    root: &Path,
    config: &Config,
    ignore: &GlobSet,
    managed_documents: usize,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    let declared = config.document_contracts.maturity.declared;
    let mut project_lines = 0usize;
    let mut project_bytes = 0u64;
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| entry.depth() == 0 || entry.file_name() != ".git")
    {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let rel = entry.path().strip_prefix(root).unwrap_or(entry.path());
        if ignore.is_match(rel) {
            continue;
        }
        project_bytes = project_bytes.saturating_add(entry.metadata()?.len());
        if let Ok(text) = std::fs::read_to_string(entry.path()) {
            project_lines = project_lines.saturating_add(text.lines().count());
        }
    }

    let recommended = config
        .document_contracts
        .maturity
        .recommendations
        .iter()
        .filter(|recommendation| recommendation.level > declared)
        .filter(|recommendation| {
            let has_signal = recommendation.min_project_lines.is_some()
                || recommendation.min_project_bytes.is_some()
                || recommendation.min_managed_documents.is_some();
            has_signal
                && recommendation
                    .min_project_lines
                    .is_none_or(|minimum| project_lines >= minimum)
                && recommendation
                    .min_project_bytes
                    .is_none_or(|minimum| project_bytes >= minimum)
                && recommendation
                    .min_managed_documents
                    .is_none_or(|minimum| managed_documents >= minimum)
        })
        .map(|recommendation| recommendation.level)
        .max();

    if let Some(level) = recommended {
        diagnostics.push(Diagnostic::new(
            "DH_MATURITY_001",
            Severity::Info,
            ".",
            format!(
                "Project signals recommend document governance maturity {:?}; declared {:?} ({} lines, {} bytes, {} managed docs).",
                level, declared, project_lines, project_bytes, managed_documents
            ),
        ));
    }
    Ok(())
}
