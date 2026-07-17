use regex::Regex;
use serde_yaml::{Mapping, Value};

use crate::config::{
    DocumentKindConfig, FrontmatterFieldConfig, FrontmatterFieldType, FrontmatterInvariantOperator,
};

use super::{KindIssue, KindIssueCategory};

pub fn validate_kind_frontmatter(kind: &DocumentKindConfig, mapping: &Mapping) -> Vec<KindIssue> {
    let mut issues = Vec::new();
    let schema = &kind.frontmatter;
    let revision_key = Value::String(schema.revision_field.clone());
    match mapping.get(&revision_key).and_then(Value::as_u64) {
        Some(revision) if revision == schema.revision => {}
        Some(revision)
            if revision >= schema.compatible_from.unwrap_or(schema.revision)
                && revision < schema.revision =>
        {
            issues.push(KindIssue {
                category: KindIssueCategory::SchemaRevision,
                field: Some(schema.revision_field.clone()),
                message: format!(
                    "Document Kind '{}' uses compatible schema revision {revision}; migrate to {}.",
                    kind.id, schema.revision
                ),
                blocking: false,
            });
        }
        Some(revision) => issues.push(KindIssue {
            category: KindIssueCategory::SchemaRevision,
            field: Some(schema.revision_field.clone()),
            message: format!(
                "Document Kind '{}' schema revision {revision} is outside compatibility window {}..={}.",
                kind.id,
                schema.compatible_from.unwrap_or(schema.revision),
                schema.revision
            ),
            blocking: true,
        }),
        None => issues.push(KindIssue {
            category: KindIssueCategory::SchemaRevision,
            field: Some(schema.revision_field.clone()),
            message: format!(
                "Document Kind '{}' requires integer frontmatter field '{}'.",
                kind.id, schema.revision_field
            ),
            blocking: false,
        }),
    }

    let known = schema
        .fields
        .iter()
        .map(|field| field.id.as_str())
        .chain(std::iter::once(schema.revision_field.as_str()))
        .collect::<std::collections::BTreeSet<_>>();
    if !schema.allow_unknown_fields {
        for key in mapping.keys() {
            match key.as_str() {
                Some(field) if known.contains(field) => {}
                Some(field) => issues.push(frontmatter_issue(
                    field,
                    format!(
                        "Document Kind '{}' forbids unknown frontmatter field '{field}'.",
                        kind.id
                    ),
                )),
                None => issues.push(frontmatter_issue(
                    "<non-string>",
                    format!(
                        "Document Kind '{}' requires string frontmatter field names.",
                        kind.id
                    ),
                )),
            }
        }
    }

    for field in &schema.fields {
        let value = mapping.get(Value::String(field.id.clone()));
        if value.is_none() && field.required {
            issues.push(frontmatter_issue(
                &field.id,
                format!(
                    "Document Kind '{}' requires frontmatter field '{}'.",
                    kind.id, field.id
                ),
            ));
            continue;
        }
        let Some(value) = value else { continue };
        validate_field_value(kind, field, value, &mut issues);
    }

    for condition in &schema.conditions {
        let active = mapping.get(Value::String(condition.when.field.clone()))
            == Some(&condition.when.equals);
        if !active {
            continue;
        }
        for field in &condition.required {
            if !mapping.contains_key(Value::String(field.clone())) {
                issues.push(frontmatter_issue(
                    field,
                    format!(
                        "Document Kind '{}' requires field '{}' when '{}' equals {}.",
                        kind.id,
                        field,
                        condition.when.field,
                        yaml_inline(&condition.when.equals)
                    ),
                ));
            }
        }
        for field in &condition.forbidden {
            if mapping.contains_key(Value::String(field.clone())) {
                issues.push(frontmatter_issue(
                    field,
                    format!(
                        "Document Kind '{}' forbids field '{}' when '{}' equals {}.",
                        kind.id,
                        field,
                        condition.when.field,
                        yaml_inline(&condition.when.equals)
                    ),
                ));
            }
        }
    }

    for invariant in &schema.invariants {
        let left = mapping.get(Value::String(invariant.left.clone()));
        let right = mapping.get(Value::String(invariant.right.clone()));
        let (Some(left), Some(right)) = (left, right) else {
            continue;
        };
        let satisfied = match invariant.operator {
            FrontmatterInvariantOperator::Equals => left == right,
            FrontmatterInvariantOperator::NotEquals => left != right,
        };
        if !satisfied {
            issues.push(frontmatter_issue(
                &invariant.left,
                format!(
                    "Document Kind '{}' requires '{}' to {} '{}'.",
                    kind.id,
                    invariant.left,
                    match invariant.operator {
                        FrontmatterInvariantOperator::Equals => "equal",
                        FrontmatterInvariantOperator::NotEquals => "differ from",
                    },
                    invariant.right
                ),
            ));
        }
    }
    issues
}

pub fn parse_frontmatter_mapping(text: &str) -> anyhow::Result<Mapping> {
    let Some(rest) = text.strip_prefix("---\n") else {
        anyhow::bail!("document requires YAML frontmatter");
    };
    let Some((yaml, _)) = rest.split_once("\n---") else {
        anyhow::bail!("document has unterminated YAML frontmatter");
    };
    match serde_yaml::from_str::<Value>(yaml)? {
        Value::Mapping(mapping) => Ok(mapping),
        _ => anyhow::bail!("document frontmatter must be a YAML mapping"),
    }
}

pub(super) fn validate_field_value(
    kind: &DocumentKindConfig,
    field: &FrontmatterFieldConfig,
    value: &Value,
    issues: &mut Vec<KindIssue>,
) {
    let type_valid = match field.field_type {
        FrontmatterFieldType::String => value.is_string(),
        FrontmatterFieldType::Integer => value.as_i64().is_some() || value.as_u64().is_some(),
        FrontmatterFieldType::Number => value.as_f64().is_some(),
        FrontmatterFieldType::Boolean => value.as_bool().is_some(),
        FrontmatterFieldType::StringList => value
            .as_sequence()
            .is_some_and(|values| values.iter().all(Value::is_string)),
    };
    if !type_valid {
        issues.push(frontmatter_issue(
            &field.id,
            format!(
                "Document Kind '{}' field '{}' must have type {:?}.",
                kind.id, field.id, field.field_type
            ),
        ));
        return;
    }
    if !field.values.is_empty() && !field.values.contains(value) {
        issues.push(frontmatter_issue(
            &field.id,
            format!(
                "Document Kind '{}' field '{}' value {} is outside its enum.",
                kind.id,
                field.id,
                yaml_inline(value)
            ),
        ));
    }
    if let Some(pattern) = &field.format
        && let Some(value) = value.as_str()
        && Regex::new(pattern).is_ok_and(|regex| !regex.is_match(value))
    {
        issues.push(frontmatter_issue(
            &field.id,
            format!(
                "Document Kind '{}' field '{}' value '{}' does not match format '{}'.",
                kind.id, field.id, value, pattern
            ),
        ));
    }
}

pub(super) fn registry_issue(message: String) -> KindIssue {
    KindIssue {
        category: KindIssueCategory::Registry,
        field: None,
        message,
        blocking: true,
    }
}

fn frontmatter_issue(field: &str, message: String) -> KindIssue {
    KindIssue {
        category: KindIssueCategory::Frontmatter,
        field: Some(field.to_owned()),
        message,
        blocking: true,
    }
}

fn yaml_inline(value: &Value) -> String {
    serde_yaml::to_string(value)
        .unwrap_or_default()
        .trim()
        .to_owned()
}
