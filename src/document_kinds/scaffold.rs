use std::collections::BTreeSet;
use std::path::{Component, Path};

use anyhow::{Result, anyhow, bail};
use globset::{Glob, GlobSetBuilder};
use regex::Regex;
use serde_yaml::{Mapping, Value};

use crate::Config;
use crate::config::{
    DocumentKindConfig, DocumentProfileConfig, DocumentTemplateConfig, FrontmatterFieldSource,
    FrontmatterFieldType, RequiredSectionConfig, SlugNormalization, SlugRenamePolicy,
    SlugSourceConfig,
};

use super::registry::validate_document_kind_registry;
use super::schema::validate_kind_frontmatter;
use super::{KindIssueCategory, ScaffoldDocumentPlan, ScaffoldDocumentRequest};

pub fn plan_scaffold_document(
    config: &Config,
    request: &ScaffoldDocumentRequest,
) -> Result<ScaffoldDocumentPlan> {
    let kind = config
        .document_kinds
        .iter()
        .find(|kind| kind.id == request.kind)
        .ok_or_else(|| anyhow!("unknown Document Kind '{}'", request.kind))?;
    let registry_issues = validate_document_kind_registry(config)
        .into_iter()
        .filter(|issue| issue.message.contains(&format!("'{}'", kind.id)))
        .collect::<Vec<_>>();
    if !registry_issues.is_empty() {
        bail!(
            "Document Kind '{}' registry is invalid: {}",
            kind.id,
            registry_issues
                .iter()
                .map(|issue| issue.message.as_str())
                .collect::<Vec<_>>()
                .join("; ")
        );
    }
    let base = config
        .docs
        .bases
        .iter()
        .find(|base| base.id == kind.base)
        .expect("validated kind base");
    let pattern = base
        .patterns
        .iter()
        .find(|pattern| pattern.id == kind.pattern)
        .expect("validated kind pattern");
    let profile = config
        .document_contracts
        .profiles
        .iter()
        .find(|profile| profile.id == kind.profile)
        .expect("validated kind profile");

    let filename = render_template(
        &kind.scaffold.filename,
        &request.identity,
        &request.slug,
        &request.locale,
    )?;
    ensure_filename(&filename)?;
    if !Regex::new(&pattern.regex)?.is_match(&filename) {
        bail!(
            "generated filename '{filename}' does not match pattern '{}'",
            kind.pattern
        );
    }
    let canonical = config
        .language_representations
        .canonical
        .as_deref()
        .unwrap_or("canonical");
    let default_root = if request.locale == canonical {
        base.root.clone()
    } else {
        base.localized_roots
            .get(&request.locale)
            .cloned()
            .ok_or_else(|| {
                anyhow!(
                    "Document Kind '{}' has no root for locale '{}'",
                    kind.id,
                    request.locale
                )
            })?
    };
    let target = request.target_dir.clone().unwrap_or(default_root);
    ensure_relative_safe(&target)?;
    let relative_path = target.join(&filename);
    if !profile_matches(&relative_path, profile)? {
        bail!(
            "generated path '{}' does not match document profile '{}'",
            relative_path.display(),
            profile.id
        );
    }

    let mapping = scaffold_frontmatter(kind, request)?;
    validate_slug_policy(config, kind, pattern, &filename, &mapping)?;
    let issues = validate_kind_frontmatter(kind, &mapping)
        .into_iter()
        .filter(|issue| issue.category != KindIssueCategory::SchemaRevision || issue.blocking)
        .collect::<Vec<_>>();
    if !issues.is_empty() {
        bail!(
            "invalid scaffold input: {}",
            issues
                .iter()
                .map(|issue| issue.message.as_str())
                .collect::<Vec<_>>()
                .join("; ")
        );
    }
    let sections = resolved_sections(config, profile);
    let content = render_document(config, kind, request, &mapping, &sections)?;
    validate_required_fields(config, profile, &content)?;
    Ok(ScaffoldDocumentPlan {
        relative_path,
        content,
    })
}

fn scaffold_frontmatter(
    kind: &DocumentKindConfig,
    request: &ScaffoldDocumentRequest,
) -> Result<Mapping> {
    let schema = &kind.frontmatter;
    let mut mapping = Mapping::new();
    mapping.insert(
        Value::String(schema.revision_field.clone()),
        serde_yaml::to_value(schema.revision)?,
    );
    let known_inputs = schema
        .fields
        .iter()
        .filter(|field| field.source == FrontmatterFieldSource::Input)
        .map(|field| field.id.as_str())
        .collect::<BTreeSet<_>>();
    if !schema.allow_unknown_fields {
        for input in request.fields.keys() {
            if !known_inputs.contains(input.as_str()) {
                bail!(
                    "unknown scaffold field '{input}' for Document Kind '{}'",
                    kind.id
                );
            }
        }
    }
    for field in &schema.fields {
        let supplied = match field.source {
            FrontmatterFieldSource::Identity => Some(Value::String(request.identity.clone())),
            FrontmatterFieldSource::Slug => Some(Value::String(request.slug.clone())),
            FrontmatterFieldSource::Locale => Some(Value::String(request.locale.clone())),
            FrontmatterFieldSource::Input => request
                .fields
                .get(&field.id)
                .map(|value| parse_input_value(field.field_type, value))
                .transpose()?,
        };
        if let Some(value) = supplied.or_else(|| field.default.clone()) {
            mapping.insert(Value::String(field.id.clone()), value);
        }
    }
    if schema.allow_unknown_fields {
        for (field, value) in &request.fields {
            if !mapping.contains_key(Value::String(field.clone()))
                && !known_inputs.contains(field.as_str())
            {
                mapping.insert(Value::String(field.clone()), Value::String(value.clone()));
            }
        }
    }
    Ok(mapping)
}

fn parse_input_value(field_type: FrontmatterFieldType, input: &str) -> Result<Value> {
    Ok(match field_type {
        FrontmatterFieldType::String => Value::String(input.to_owned()),
        FrontmatterFieldType::Integer => serde_yaml::to_value(input.parse::<i64>()?)?,
        FrontmatterFieldType::Number => serde_yaml::to_value(input.parse::<f64>()?)?,
        FrontmatterFieldType::Boolean => serde_yaml::to_value(input.parse::<bool>()?)?,
        FrontmatterFieldType::StringList => Value::Sequence(
            input
                .split(',')
                .map(|value| Value::String(value.trim().to_owned()))
                .collect(),
        ),
    })
}

fn render_document(
    config: &Config,
    kind: &DocumentKindConfig,
    request: &ScaffoldDocumentRequest,
    mapping: &Mapping,
    sections: &[RequiredSectionConfig],
) -> Result<String> {
    let yaml = serde_yaml::to_string(mapping)?;
    let title = render_template(
        &kind.scaffold.title,
        &request.identity,
        &request.slug,
        &request.locale,
    )?;
    let canonical = config
        .language_representations
        .canonical
        .as_deref()
        .unwrap_or("canonical");
    let mut content = format!("---\n{yaml}---\n\n# {title}\n");
    for section in sections {
        let heading = kind
            .scaffold
            .section_headings
            .get(&section.id)
            .and_then(|headings| {
                headings
                    .get(&request.locale)
                    .or_else(|| headings.get(canonical))
            })
            .cloned()
            .or_else(|| section.headings.first().cloned())
            .ok_or_else(|| anyhow!("section '{}' has no accepted heading", section.id))?;
        if !section.headings.contains(&heading) {
            bail!(
                "scaffold heading '{heading}' for section '{}' is not accepted by profile '{}'",
                section.id,
                kind.profile
            );
        }
        content.push_str(&format!("\n## {heading}\n"));
    }
    Ok(content)
}

pub(super) fn resolved_sections(
    config: &Config,
    profile: &DocumentProfileConfig,
) -> Vec<RequiredSectionConfig> {
    let mut sections = profile
        .template
        .as_deref()
        .and_then(|id| {
            config
                .document_contracts
                .templates
                .iter()
                .find(|template| template.id == id)
        })
        .map(|template: &DocumentTemplateConfig| template.contract.required_sections.clone())
        .unwrap_or_default();
    sections.extend(profile.contract.required_sections.clone());
    sections
}

fn validate_required_fields(
    config: &Config,
    profile: &DocumentProfileConfig,
    content: &str,
) -> Result<()> {
    let mut fields = profile
        .template
        .as_deref()
        .and_then(|id| {
            config
                .document_contracts
                .templates
                .iter()
                .find(|template| template.id == id)
        })
        .map(|template| template.contract.required_fields.clone())
        .unwrap_or_default();
    fields.extend(profile.contract.required_fields.clone());
    for field in fields {
        if !Regex::new(&field.pattern)?.is_match(content) {
            bail!(
                "generated document cannot satisfy required field '{}' in profile '{}'",
                field.id,
                profile.id
            );
        }
    }
    Ok(())
}

fn validate_slug_policy(
    config: &Config,
    kind: &DocumentKindConfig,
    filename_pattern: &crate::config::FilenamePatternConfig,
    filename: &str,
    mapping: &Mapping,
) -> Result<()> {
    let Some(schema) = config
        .docs
        .slug_schemas
        .iter()
        .find(|schema| schema.document_kind == kind.id)
    else {
        return Ok(());
    };
    let original = match &schema.source {
        SlugSourceConfig::Filename { capture } => Regex::new(&filename_pattern.regex)?
            .captures(filename)
            .and_then(|captures| captures.name(capture))
            .map(|value| value.as_str()),
        SlugSourceConfig::Frontmatter { field } | SlugSourceConfig::StableId { field } => mapping
            .get(Value::String(field.clone()))
            .and_then(Value::as_str),
    }
    .ok_or_else(|| anyhow!("generated document cannot resolve its authoritative slug"))?;
    let normalized = normalize_slug(original, schema.normalization);
    let length = normalized.chars().count();
    if !Regex::new(&schema.pattern)?.is_match(&normalized)
        || schema.min_length.is_some_and(|min| length < min)
        || schema.max_length.is_some_and(|max| length > max)
        || schema
            .reserved
            .iter()
            .any(|value| normalize_slug(value, schema.normalization) == normalized)
    {
        bail!(
            "generated slug '{original}' normalizes to '{normalized}' and violates the Document Kind slug Schema"
        );
    }
    if schema.rename_policy != SlugRenamePolicy::AllowPathBreak
        && mapping
            .get(Value::String(schema.identity_field.clone()))
            .and_then(Value::as_str)
            .is_none()
    {
        bail!(
            "generated document lacks stable slug identity field '{}'",
            schema.identity_field
        );
    }
    if schema.rename_policy == SlugRenamePolicy::RequireAlias
        && mapping
            .get(Value::String(schema.aliases_field.clone()))
            .and_then(Value::as_sequence)
            .is_none_or(|aliases| aliases.is_empty())
    {
        bail!(
            "Document Kind is in requireAlias migration mode but '{}' is empty",
            schema.aliases_field
        );
    }
    Ok(())
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

fn profile_matches(path: &Path, profile: &DocumentProfileConfig) -> Result<bool> {
    let filename = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    let path_matches = if profile.matcher.paths.is_empty() {
        true
    } else {
        let mut builder = GlobSetBuilder::new();
        for pattern in &profile.matcher.paths {
            builder.add(Glob::new(pattern)?);
        }
        builder.build()?.is_match(path)
    };
    let filename_matches = profile.matcher.filenames.is_empty()
        || profile
            .matcher
            .filenames
            .iter()
            .any(|pattern| Regex::new(pattern).is_ok_and(|regex| regex.is_match(filename)));
    Ok(path_matches && filename_matches)
}

fn render_template(template: &str, identity: &str, slug: &str, locale: &str) -> Result<String> {
    if let Some(placeholder) = unknown_placeholder(template) {
        bail!("unknown scaffold placeholder '{{{placeholder}}}'");
    }
    Ok(template
        .replace("{identity}", identity)
        .replace("{slug}", slug)
        .replace("{locale}", locale))
}

pub(super) fn unknown_placeholder(template: &str) -> Option<String> {
    let placeholder = Regex::new(r"\{([^{}]+)\}").expect("static placeholder regex");
    placeholder.captures_iter(template).find_map(|capture| {
        let value = capture.get(1)?.as_str();
        (!matches!(value, "identity" | "slug" | "locale")).then(|| value.to_owned())
    })
}

fn ensure_filename(filename: &str) -> Result<()> {
    let path = Path::new(filename);
    if filename.is_empty()
        || path.components().count() != 1
        || !matches!(path.components().next(), Some(Component::Normal(_)))
    {
        bail!("generated filename must be one safe relative path component");
    }
    Ok(())
}

fn ensure_relative_safe(path: &Path) -> Result<()> {
    if path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        bail!("target directory must be a safe project-relative path");
    }
    Ok(())
}
