use std::collections::BTreeSet;

use regex::Regex;

use crate::Config;
use crate::config::{DocumentKindConfig, SlugSourceConfig};

use super::KindIssue;
use super::scaffold::{resolved_sections, unknown_placeholder};
use super::schema::{registry_issue, validate_field_value};

pub fn validate_document_kind_registry(config: &Config) -> Vec<KindIssue> {
    let mut issues = Vec::new();
    let mut ids = BTreeSet::new();
    for kind in &config.document_kinds {
        if !ids.insert(kind.id.as_str()) {
            issues.push(registry_issue(format!(
                "Document Kind '{}' is declared more than once.",
                kind.id
            )));
        }
        if !valid_identity(&kind.id) {
            issues.push(registry_issue(format!(
                "Document Kind identity '{}' is invalid.",
                kind.id
            )));
        }
        let base = config.docs.bases.iter().find(|base| base.id == kind.base);
        let pattern = base.and_then(|base| {
            base.patterns
                .iter()
                .find(|pattern| pattern.id == kind.pattern)
        });
        if base.is_none() {
            issues.push(registry_issue(format!(
                "Document Kind '{}' references unknown docs base '{}'.",
                kind.id, kind.base
            )));
        } else if pattern.is_none() {
            issues.push(registry_issue(format!(
                "Document Kind '{}' references unknown filename pattern '{}' in base '{}'.",
                kind.id, kind.pattern, kind.base
            )));
        } else if pattern.is_some_and(|pattern| pattern.document_kind != kind.id) {
            issues.push(registry_issue(format!(
                "Document Kind '{}' references pattern '{}', which declares documentKind '{}'.",
                kind.id,
                kind.pattern,
                pattern.expect("checked pattern").document_kind
            )));
        }
        if config
            .document_contracts
            .profiles
            .iter()
            .all(|profile| profile.id != kind.profile)
        {
            issues.push(registry_issue(format!(
                "Document Kind '{}' references unknown document profile '{}'.",
                kind.id, kind.profile
            )));
        }
        validate_profile_template_binding(config, kind, &mut issues);
        validate_slug_binding(config, kind, pattern, &mut issues);
        validate_frontmatter_schema(kind, &mut issues);
        validate_scaffold_config(config, kind, &mut issues);
    }
    issues
}

fn validate_frontmatter_schema(kind: &DocumentKindConfig, issues: &mut Vec<KindIssue>) {
    let schema = &kind.frontmatter;
    let compatible_from = schema.compatible_from.unwrap_or(schema.revision);
    if schema.revision == 0 || compatible_from == 0 || compatible_from > schema.revision {
        issues.push(registry_issue(format!(
            "Document Kind '{}' has invalid frontmatter revision window {}..={}.",
            kind.id, compatible_from, schema.revision
        )));
    }
    let mut fields = BTreeSet::new();
    if !valid_identity(&schema.revision_field) {
        issues.push(registry_issue(format!(
            "Document Kind '{}' revisionField '{}' is invalid.",
            kind.id, schema.revision_field
        )));
    }
    fields.insert(schema.revision_field.as_str());
    for field in &schema.fields {
        if !valid_identity(&field.id) {
            issues.push(registry_issue(format!(
                "Document Kind '{}' frontmatter field identity '{}' is invalid.",
                kind.id, field.id
            )));
        }
        if !fields.insert(field.id.as_str()) {
            issues.push(registry_issue(format!(
                "Document Kind '{}' repeats frontmatter field '{}'.",
                kind.id, field.id
            )));
        }
        if let Some(pattern) = &field.format
            && let Err(error) = Regex::new(pattern)
        {
            issues.push(registry_issue(format!(
                "Document Kind '{}' field '{}' has invalid format '{pattern}': {error}.",
                kind.id, field.id
            )));
        }
        if let Some(default) = &field.default {
            validate_field_value(kind, field, default, issues);
        }
    }
    for condition in &schema.conditions {
        for field in std::iter::once(&condition.when.field)
            .chain(&condition.required)
            .chain(&condition.forbidden)
        {
            if !fields.contains(field.as_str()) {
                issues.push(registry_issue(format!(
                    "Document Kind '{}' condition references unknown field '{}'.",
                    kind.id, field
                )));
            }
        }
    }
    for invariant in &schema.invariants {
        for field in [&invariant.left, &invariant.right] {
            if !fields.contains(field.as_str()) {
                issues.push(registry_issue(format!(
                    "Document Kind '{}' invariant references unknown field '{}'.",
                    kind.id, field
                )));
            }
        }
    }
}

fn validate_profile_template_binding(
    config: &Config,
    kind: &DocumentKindConfig,
    issues: &mut Vec<KindIssue>,
) {
    let Some(profile) = config
        .document_contracts
        .profiles
        .iter()
        .find(|profile| profile.id == kind.profile)
    else {
        return;
    };
    let Some(template_id) = profile.template.as_deref() else {
        issues.push(registry_issue(format!(
            "Document Kind '{}' profile '{}' must bind a revisioned template.",
            kind.id, profile.id
        )));
        return;
    };
    let Some(template) = config
        .document_contracts
        .templates
        .iter()
        .find(|template| template.id == template_id)
    else {
        return;
    };
    let Some(revision) = template.revision else {
        return;
    };
    if profile.template_revision != Some(revision) {
        issues.push(registry_issue(format!(
            "Document Kind '{}' profile '{}' must pin current template '{}' revision {}; run migrate-kinds.",
            kind.id, profile.id, template.id, revision
        )));
    }
}

fn validate_slug_binding(
    config: &Config,
    kind: &DocumentKindConfig,
    pattern: Option<&crate::config::FilenamePatternConfig>,
    issues: &mut Vec<KindIssue>,
) {
    let schemas = config
        .docs
        .slug_schemas
        .iter()
        .filter(|schema| schema.document_kind == kind.id)
        .collect::<Vec<_>>();
    if schemas.len() > 1 {
        issues.push(registry_issue(format!(
            "Document Kind '{}' resolves more than one slug Schema.",
            kind.id
        )));
        return;
    }
    let Some(schema) = schemas.first() else {
        return;
    };
    let fields = kind
        .frontmatter
        .fields
        .iter()
        .map(|field| field.id.as_str())
        .collect::<BTreeSet<_>>();
    match &schema.source {
        SlugSourceConfig::Filename { capture } => {
            let capture_exists = pattern
                .and_then(|pattern| Regex::new(&pattern.regex).ok())
                .is_some_and(|regex| regex.capture_names().flatten().any(|name| name == capture));
            if !capture_exists {
                issues.push(registry_issue(format!(
                    "Document Kind '{}' slug Schema capture '{}' is absent from pattern '{}'.",
                    kind.id, capture, kind.pattern
                )));
            }
        }
        SlugSourceConfig::Frontmatter { field } | SlugSourceConfig::StableId { field } => {
            if !fields.contains(field.as_str()) {
                issues.push(registry_issue(format!(
                    "Document Kind '{}' slug Schema source field '{}' is absent from its frontmatter Schema.",
                    kind.id, field
                )));
            }
        }
    }
    if !fields.contains(schema.identity_field.as_str()) {
        issues.push(registry_issue(format!(
            "Document Kind '{}' slug identity field '{}' is absent from its frontmatter Schema.",
            kind.id, schema.identity_field
        )));
    }
}

fn valid_identity(identity: &str) -> bool {
    !identity.is_empty()
        && identity
            .chars()
            .all(|value| value.is_ascii_alphanumeric() || matches!(value, '.' | '_' | '-'))
}

fn validate_scaffold_config(
    config: &Config,
    kind: &DocumentKindConfig,
    issues: &mut Vec<KindIssue>,
) {
    for (label, template) in [
        ("filename", kind.scaffold.filename.as_str()),
        ("title", kind.scaffold.title.as_str()),
    ] {
        if let Some(placeholder) = unknown_placeholder(template) {
            issues.push(registry_issue(format!(
                "Document Kind '{}' {label} template uses unknown placeholder '{{{placeholder}}}'.",
                kind.id
            )));
        }
    }
    let Some(profile) = config
        .document_contracts
        .profiles
        .iter()
        .find(|profile| profile.id == kind.profile)
    else {
        return;
    };
    let sections = resolved_sections(config, profile)
        .into_iter()
        .map(|section| (section.id, section.headings))
        .collect::<std::collections::BTreeMap<_, _>>();
    for (section, localized) in &kind.scaffold.section_headings {
        let Some(accepted) = sections.get(section) else {
            issues.push(registry_issue(format!(
                "Document Kind '{}' scaffold references unknown section '{}'.",
                kind.id, section
            )));
            continue;
        };
        for (locale, heading) in localized {
            if !accepted.contains(heading) {
                issues.push(registry_issue(format!(
                    "Document Kind '{}' scaffold heading '{}' for section '{}' and locale '{}' is not accepted by profile '{}'.",
                    kind.id, heading, section, locale, kind.profile
                )));
            }
        }
    }
}
