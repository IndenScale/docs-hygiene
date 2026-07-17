fn check_document_kinds(
    root: &Path,
    config: &Config,
    docs: &[DocFile],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    for issue in crate::document_kinds::validate_document_kind_registry(config) {
        diagnostics.push(Diagnostic::new(
            "DH_KIND_001",
            Severity::Error,
            "docs-hygiene.yml",
            issue.message,
        ));
    }

    let kinds = config
        .document_kinds
        .iter()
        .map(|kind| (kind.id.as_str(), kind))
        .collect::<BTreeMap<_, _>>();
    for doc in docs {
        let Some(kind) = kinds.get(doc.document_kind.as_str()).copied() else {
            continue;
        };
        if doc.base_id != kind.base || doc.pattern_id != kind.pattern {
            diagnostics.push(Diagnostic::new(
                "DH_KIND_001",
                Severity::Error,
                doc.rel.display().to_string(),
                format!(
                    "Document Kind '{}' is owned by base/pattern '{}/{}', but this file was classified by '{}/{}'.",
                    kind.id, kind.base, kind.pattern, doc.base_id, doc.pattern_id
                ),
            ));
            continue;
        }
        let Some(profile) = first_matching_profile_config(&doc.rel, config)? else {
            diagnostics.push(Diagnostic::new(
                "DH_KIND_001",
                Severity::Error,
                doc.rel.display().to_string(),
                format!(
                    "Document Kind '{}' requires profile '{}', but no document profile owns this path.",
                    kind.id, kind.profile
                ),
            ));
            continue;
        };
        if profile.id != kind.profile {
            diagnostics.push(Diagnostic::new(
                "DH_KIND_001",
                Severity::Error,
                doc.rel.display().to_string(),
                format!(
                    "Document Kind '{}' requires profile '{}', but first-match ownership resolves to '{}'.",
                    kind.id, kind.profile, profile.id
                ),
            ));
        }

        let text = std::fs::read_to_string(root.join(&doc.rel))?;
        let mapping = match crate::document_kinds::parse_frontmatter_mapping(&text) {
            Ok(mapping) => mapping,
            Err(error) => {
                diagnostics.push(Diagnostic::new(
                    "DH_FRONTMATTER_001",
                    Severity::Error,
                    doc.rel.display().to_string(),
                    format!("Document Kind '{}' {error}.", kind.id),
                ));
                continue;
            }
        };
        for issue in crate::document_kinds::validate_kind_frontmatter(kind, &mapping) {
            let code = match issue.category {
                crate::document_kinds::KindIssueCategory::SchemaRevision => "DH_KIND_002",
                crate::document_kinds::KindIssueCategory::Registry => "DH_KIND_001",
                crate::document_kinds::KindIssueCategory::Frontmatter => "DH_FRONTMATTER_001",
            };
            let severity = if issue.blocking {
                Severity::Error
            } else {
                Severity::Warning
            };
            let mut diagnostic = Diagnostic::new(
                code,
                severity,
                doc.rel.display().to_string(),
                issue.message,
            );
            if let Some(field) = issue.field
                && let Some(line) = frontmatter_field_line(&text, &field)
            {
                diagnostic = diagnostic.at_line(line);
            }
            diagnostics.push(diagnostic);
        }
    }
    Ok(())
}

fn first_matching_profile_config<'a>(
    rel: &Path,
    config: &'a Config,
) -> Result<Option<&'a DocumentProfileConfig>> {
    let filename = rel
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    for profile in &config.document_contracts.profiles {
        let path_matches = if profile.matcher.paths.is_empty() {
            true
        } else {
            let mut builder = GlobSetBuilder::new();
            for pattern in &profile.matcher.paths {
                builder.add(Glob::new(pattern)?);
            }
            builder.build()?.is_match(rel)
        };
        let filename_matches = profile.matcher.filenames.is_empty()
            || profile
                .matcher
                .filenames
                .iter()
                .any(|pattern| Regex::new(pattern).is_ok_and(|regex| regex.is_match(filename)));
        if path_matches && filename_matches {
            return Ok(Some(profile));
        }
    }
    Ok(None)
}

fn frontmatter_field_line(text: &str, field: &str) -> Option<usize> {
    for (index, line) in text.lines().enumerate().skip(1) {
        if line == "---" {
            break;
        }
        if line.trim_start().starts_with(&format!("{field}:")) {
            return Some(index + 1);
        }
    }
    None
}
