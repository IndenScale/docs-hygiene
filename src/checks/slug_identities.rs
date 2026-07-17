#[derive(Debug)]
struct SlugRecord<'a> {
    doc: &'a DocFile,
    identity: String,
    original: String,
    normalized: String,
    aliases: Vec<(String, String)>,
}

#[derive(Debug)]
struct SlugIndexEntry<'a> {
    record: &'a SlugRecord<'a>,
    original: &'a str,
    alias: bool,
}

fn check_slug_identities(
    root: &Path,
    config: &Config,
    docs: &[DocFile],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    if config.docs.slug_schemas.is_empty() {
        return Ok(());
    }

    let schemas = valid_slug_schemas(config, diagnostics);
    let bases = normalized_bases(config);
    let mut records = Vec::new();
    for doc in docs {
        let Some(schema) = schemas.get(doc.document_kind.as_str()).copied() else {
            continue;
        };
        let text = std::fs::read_to_string(root.join(&doc.rel))?;
        let frontmatter = markdown_frontmatter(&text)
            .and_then(|yaml| serde_yaml::from_str::<serde_yaml::Value>(yaml).ok())
            .and_then(|value| value.as_mapping().cloned())
            .unwrap_or_default();
        if let Some(record) = slug_record(doc, schema, &bases, &frontmatter, diagnostics) {
            records.push(record);
        }
    }

    check_slug_representation_parity(&records, diagnostics);
    check_slug_conflicts(&records, diagnostics);
    Ok(())
}

fn valid_slug_schemas<'a>(
    config: &'a Config,
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeMap<&'a str, &'a SlugSchemaConfig> {
    let mut schemas = BTreeMap::new();
    for schema in &config.docs.slug_schemas {
        if schemas.insert(schema.document_kind.as_str(), schema).is_some() {
            diagnostics.push(slug_diagnostic(
                "docs-hygiene.yml",
                &schema.document_kind,
                &schema.document_kind,
                &schema.document_kind,
                None,
                format!(
                    "Document Kind '{}' declares more than one slug Schema; keep exactly one authoritative source.",
                    schema.document_kind
                ),
                "Remove the duplicate docs.slugSchemas entry.".to_owned(),
            ));
        }
        if let Err(error) = Regex::new(&schema.pattern) {
            diagnostics.push(slug_diagnostic(
                "docs-hygiene.yml",
                &schema.document_kind,
                &schema.pattern,
                &schema.pattern,
                None,
                format!(
                    "Document Kind '{}' has invalid slug pattern '{}': {error}.",
                    schema.document_kind, schema.pattern
                ),
                "Replace pattern with a valid, anchored regular expression.".to_owned(),
            ));
        }
        if schema
            .min_length
            .zip(schema.max_length)
            .is_some_and(|(min, max)| min > max)
        {
            diagnostics.push(slug_diagnostic(
                "docs-hygiene.yml",
                &schema.document_kind,
                &schema.document_kind,
                &schema.document_kind,
                None,
                format!(
                    "Document Kind '{}' has minLength greater than maxLength.",
                    schema.document_kind
                ),
                "Set minLength less than or equal to maxLength.".to_owned(),
            ));
        }
    }
    schemas
}

fn slug_record<'a>(
    doc: &'a DocFile,
    schema: &SlugSchemaConfig,
    bases: &[NormalizedBase],
    frontmatter: &serde_yaml::Mapping,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<SlugRecord<'a>> {
    let identity = yaml_string(frontmatter, &schema.identity_field);
    if identity.is_none() && schema.rename_policy != SlugRenamePolicy::AllowPathBreak {
        diagnostics.push(slug_diagnostic(
            &doc.rel.display().to_string(),
            &schema.document_kind,
            "",
            "",
            None,
            format!(
                "Document Kind '{}' requires stable identity field '{}' so file renames do not change governance identity.",
                schema.document_kind, schema.identity_field
            ),
            format!(
                "Add '{}' to YAML frontmatter, or explicitly choose renamePolicy: allowPathBreak.",
                schema.identity_field
            ),
        ));
    }

    let original = match &schema.source {
        SlugSourceConfig::Frontmatter { field } | SlugSourceConfig::StableId { field } => {
            yaml_string(frontmatter, field)
        }
        SlugSourceConfig::Filename { capture } => filename_slug(doc, bases, capture),
    };
    let Some(original) = original else {
        diagnostics.push(slug_diagnostic(
            &doc.rel.display().to_string(),
            &schema.document_kind,
            "",
            "",
            None,
            format!(
                "Document Kind '{}' cannot resolve its authoritative slug from {}.",
                schema.document_kind,
                slug_source_label(&schema.source)
            ),
            format!(
                "Provide the configured {} source value.",
                slug_source_label(&schema.source)
            ),
        ));
        return None;
    };
    let normalized = normalize_slug(&original, schema.normalization);
    validate_slug_value(doc, schema, &original, &normalized, false, diagnostics);

    let aliases = yaml_strings(frontmatter, &schema.aliases_field)
        .into_iter()
        .map(|alias| {
            let normalized = normalize_slug(&alias, schema.normalization);
            validate_slug_value(doc, schema, &alias, &normalized, true, diagnostics);
            (alias, normalized)
        })
        .collect::<Vec<_>>();
    if schema.rename_policy == SlugRenamePolicy::RequireAlias && aliases.is_empty() {
        diagnostics.push(slug_diagnostic(
            &doc.rel.display().to_string(),
            &schema.document_kind,
            &original,
            &normalized,
            None,
            format!(
                "Document Kind '{}' is in requireAlias migration mode but '{}' is empty.",
                schema.document_kind, schema.aliases_field
            ),
            format!(
                "Add the former slug to frontmatter field '{}', or finish migration by selecting renamePolicy: stableIdentity.",
                schema.aliases_field
            ),
        ));
    }

    Some(SlugRecord {
        doc,
        identity: identity.unwrap_or_else(|| doc.rel.display().to_string()),
        original,
        normalized,
        aliases,
    })
}

fn filename_slug(doc: &DocFile, bases: &[NormalizedBase], capture: &str) -> Option<String> {
    let pattern = bases
        .iter()
        .find(|base| base.id == doc.base_id)?
        .patterns
        .iter()
        .find(|pattern| pattern.id == doc.pattern_id)?;
    let filename = doc.rel.file_name()?.to_str()?;
    Regex::new(&pattern.regex)
        .ok()?
        .captures(filename)?
        .name(capture)
        .map(|value| value.as_str().to_owned())
}

fn validate_slug_value(
    doc: &DocFile,
    schema: &SlugSchemaConfig,
    original: &str,
    normalized: &str,
    alias: bool,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let length = normalized.chars().count();
    let invalid_pattern = Regex::new(&schema.pattern).is_ok_and(|regex| !regex.is_match(normalized));
    let invalid_length = schema.min_length.is_some_and(|min| length < min)
        || schema.max_length.is_some_and(|max| length > max);
    let reserved = schema
        .reserved
        .iter()
        .any(|value| normalize_slug(value, schema.normalization) == normalized);
    if !invalid_pattern && !invalid_length && !reserved {
        return;
    }

    let mut reasons = Vec::new();
    if invalid_pattern {
        reasons.push(format!("does not match pattern '{}'", schema.pattern));
    }
    if invalid_length {
        reasons.push(format!(
            "has normalized length {length} outside configured bounds {}..{}",
            schema.min_length.map_or("unbounded".to_owned(), |v| v.to_string()),
            schema.max_length.map_or("unbounded".to_owned(), |v| v.to_string())
        ));
    }
    if reserved {
        reasons.push("is reserved".to_owned());
    }
    let value_kind = if alias { "alias" } else { "slug" };
    diagnostics.push(slug_diagnostic(
        &doc.rel.display().to_string(),
        &schema.document_kind,
        original,
        normalized,
        None,
        format!(
            "Document Kind '{}' {value_kind} '{}' normalizes to '{}' and {}.",
            schema.document_kind,
            original,
            normalized,
            reasons.join("; ")
        ),
        "Choose a non-reserved value that satisfies the configured character set and length bounds."
            .to_owned(),
    ));
}

fn check_slug_representation_parity(
    records: &[SlugRecord<'_>],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut identities = BTreeMap::<(&str, &str), Vec<&SlugRecord<'_>>>::new();
    for record in records {
        identities
            .entry((&record.doc.document_kind, &record.identity))
            .or_default()
            .push(record);
    }
    for ((kind, identity), group) in identities {
        let Some(first) = group.first() else { continue };
        for record in group.iter().skip(1) {
            if record.normalized == first.normalized {
                continue;
            }
            diagnostics.push(
                slug_diagnostic(
                    &record.doc.rel.display().to_string(),
                    kind,
                    &record.original,
                    &record.normalized,
                    Some(first.doc.rel.display().to_string()),
                    format!(
                        "Canonical/localized representations for identity '{identity}' disagree on normalized slug: '{}' versus '{}'.",
                        first.normalized, record.normalized
                    ),
                    "Use one authoritative slug across all language representations; translated filenames may still differ when the slug comes from frontmatter or stable ID."
                        .to_owned(),
                )
                .with_related(RelatedInformation::new(
                    first.doc.rel.display().to_string(),
                    "Other representation of the same stable identity.",
                )),
            );
        }
    }
}

fn check_slug_conflicts(records: &[SlugRecord<'_>], diagnostics: &mut Vec<Diagnostic>) {
    let mut index = BTreeMap::<(&str, &str), Vec<SlugIndexEntry<'_>>>::new();
    for record in records {
        index
            .entry((&record.doc.document_kind, &record.normalized))
            .or_default()
            .push(SlugIndexEntry {
                record,
                original: &record.original,
                alias: false,
            });
        for (original, normalized) in &record.aliases {
            index
                .entry((&record.doc.document_kind, normalized))
                .or_default()
                .push(SlugIndexEntry {
                    record,
                    original,
                    alias: true,
                });
        }
    }

    for ((kind, normalized), entries) in index {
        let Some(first) = entries.first() else {
            continue;
        };
        for entry in entries.iter().skip(1) {
            if entry.record.doc.rel == first.record.doc.rel
                || (entry.record.identity == first.record.identity
                    && entry.record.doc.lang != first.record.doc.lang)
            {
                continue;
            }
            let collision = if entry.original.eq_ignore_ascii_case(first.original)
                && entry.original != first.original
            {
                "case-folding collision"
            } else {
                "normalized collision"
            };
            diagnostics.push(
                slug_diagnostic(
                    &entry.record.doc.rel.display().to_string(),
                    kind,
                    entry.original,
                    normalized,
                    Some(first.record.doc.rel.display().to_string()),
                    format!(
                        "Document Kind '{kind}' {} '{}' has {collision} on normalized slug '{}' with identity '{}'.",
                        if entry.alias { "alias" } else { "slug" },
                        entry.original,
                        normalized,
                        first.record.identity
                    ),
                    "Choose a unique canonical slug or alias within this Document Kind.".to_owned(),
                )
                .with_related(RelatedInformation::new(
                    first.record.doc.rel.display().to_string(),
                    "Conflicting slug identity is declared here.",
                )),
            );
        }
    }
}

fn slug_diagnostic(
    path: &str,
    document_kind: &str,
    original: &str,
    normalized: &str,
    conflict_path: Option<String>,
    message: String,
    remediation: String,
) -> Diagnostic {
    Diagnostic::new("DH_SLUG_001", Severity::Error, path, message).with_data(DiagnosticData {
        original_value: original.to_owned(),
        normalized_value: normalized.to_owned(),
        document_kind: document_kind.to_owned(),
        conflict_path,
        remediation,
    })
}

fn yaml_string(mapping: &serde_yaml::Mapping, field: &str) -> Option<String> {
    mapping
        .get(serde_yaml::Value::String(field.to_owned()))
        .and_then(serde_yaml::Value::as_str)
        .map(str::to_owned)
}

fn yaml_strings(mapping: &serde_yaml::Mapping, field: &str) -> Vec<String> {
    mapping
        .get(serde_yaml::Value::String(field.to_owned()))
        .and_then(serde_yaml::Value::as_sequence)
        .into_iter()
        .flatten()
        .filter_map(serde_yaml::Value::as_str)
        .map(str::to_owned)
        .collect()
}

fn slug_source_label(source: &SlugSourceConfig) -> String {
    match source {
        SlugSourceConfig::Filename { capture } => format!("filename capture '{capture}'"),
        SlugSourceConfig::Frontmatter { field } => format!("frontmatter field '{field}'"),
        SlugSourceConfig::StableId { field } => format!("stable ID field '{field}'"),
    }
}

fn normalize_slug(value: &str, normalization: SlugNormalization) -> String {
    match normalization {
        SlugNormalization::None => value.to_owned(),
        SlugNormalization::Lowercase => value.to_lowercase(),
        SlugNormalization::LowercaseKebab => {
            let mut output = String::new();
            let mut separator = false;
            for character in value.chars().flat_map(char::to_lowercase) {
                if character.is_alphanumeric() {
                    if separator && !output.is_empty() {
                        output.push('-');
                    }
                    separator = false;
                    output.push(character);
                } else {
                    separator = true;
                }
            }
            output
        }
    }
}
