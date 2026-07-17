use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use globset::{Glob, GlobSet, GlobSetBuilder};
use regex::Regex;
use serde::Serialize;
use walkdir::WalkDir;

use crate::document_kinds::{KindIssueCategory, parse_frontmatter_mapping};
use crate::{
    Config, migrate_document_template_bindings, validate_document_kind_registry,
    validate_kind_frontmatter,
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KindMigrationChange {
    pub kind: String,
    pub path: String,
    pub from_revision: Option<u64>,
    pub to_revision: u64,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KindMigrationBlock {
    pub kind: Option<String>,
    pub path: String,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KindTemplateMigrationChange {
    pub profile: String,
    pub template: String,
    pub from_revision: Option<u64>,
    pub to_revision: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KindMigrationReport {
    pub schema_version: &'static str,
    pub schema_changes: Vec<KindMigrationChange>,
    pub template_changes: Vec<KindTemplateMigrationChange>,
    pub blocked: Vec<KindMigrationBlock>,
    pub applied: bool,
}

impl KindMigrationReport {
    pub fn is_clean(&self) -> bool {
        self.schema_changes.is_empty()
            && self.template_changes.is_empty()
            && self.blocked.is_empty()
    }
}

pub fn migrate_document_kinds(
    root: &Path,
    config: &mut Config,
    apply: bool,
) -> Result<KindMigrationReport> {
    let serialized = serde_yaml::to_string(config)?;
    let mut candidate: Config = serde_yaml::from_str(&serialized)?;
    let template_report = migrate_document_template_bindings(&mut candidate, true);
    let mut report = KindMigrationReport {
        schema_version: "docs-hygiene.kind-migration.v1",
        schema_changes: Vec::new(),
        template_changes: template_report
            .changes
            .into_iter()
            .map(|change| KindTemplateMigrationChange {
                profile: change.profile,
                template: change.template,
                from_revision: change.from_revision,
                to_revision: change.to_revision,
            })
            .collect(),
        blocked: template_report
            .blocked
            .into_iter()
            .map(|blocked| KindMigrationBlock {
                kind: None,
                path: "docs-hygiene.yml".to_owned(),
                reason: format!(
                    "template '{}'{}: {}",
                    blocked.template,
                    blocked
                        .profile
                        .map(|profile| format!(" for profile '{profile}'"))
                        .unwrap_or_default(),
                    blocked.reason
                ),
            })
            .collect(),
        applied: false,
    };

    for issue in validate_document_kind_registry(&candidate) {
        report.blocked.push(KindMigrationBlock {
            kind: None,
            path: "docs-hygiene.yml".to_owned(),
            reason: issue.message,
        });
    }

    let mut pending = BTreeMap::<PathBuf, String>::new();
    let global_ignore = compile_globs(&candidate.ignore.paths)?;
    for kind in &candidate.document_kinds {
        let Some(base) = candidate
            .docs
            .bases
            .iter()
            .find(|base| base.id == kind.base)
        else {
            continue;
        };
        let Some(pattern) = base
            .patterns
            .iter()
            .find(|pattern| pattern.id == kind.pattern)
        else {
            continue;
        };
        let filename = match Regex::new(&pattern.regex) {
            Ok(filename) => filename,
            Err(_) => continue,
        };
        let base_ignore = compile_globs(&base.ignore)?;
        let roots =
            std::iter::once(base.root.clone()).chain(base.localized_roots.values().cloned());
        for relative_root in roots {
            let docs_root = root.join(&relative_root);
            if !docs_root.exists() {
                continue;
            }
            for entry in WalkDir::new(&docs_root).sort_by_file_name() {
                let entry = entry?;
                if !entry.file_type().is_file()
                    || entry.path().extension().and_then(|value| value.to_str()) != Some("md")
                    || !entry
                        .file_name()
                        .to_str()
                        .is_some_and(|name| filename.is_match(name))
                {
                    continue;
                }
                let rel = entry.path().strip_prefix(root)?.to_path_buf();
                if global_ignore.is_match(&rel) || base_ignore.is_match(&rel) {
                    continue;
                }
                let owner = first_profile_owner(&candidate, &rel)?;
                if owner.as_deref() != Some(kind.profile.as_str()) {
                    report.blocked.push(KindMigrationBlock {
                        kind: Some(kind.id.clone()),
                        path: rel.display().to_string(),
                        reason: format!(
                            "first-match profile owner is '{}', expected '{}'",
                            owner.unwrap_or_else(|| "<none>".to_owned()),
                            kind.profile
                        ),
                    });
                    continue;
                }
                let text = std::fs::read_to_string(entry.path())?;
                let mut mapping = match parse_frontmatter_mapping(&text) {
                    Ok(mapping) => mapping,
                    Err(error) => {
                        report.blocked.push(KindMigrationBlock {
                            kind: Some(kind.id.clone()),
                            path: rel.display().to_string(),
                            reason: error.to_string(),
                        });
                        continue;
                    }
                };
                let revision_key =
                    serde_yaml::Value::String(kind.frontmatter.revision_field.clone());
                let from_revision = mapping
                    .get(&revision_key)
                    .and_then(serde_yaml::Value::as_u64);
                if from_revision == Some(kind.frontmatter.revision) {
                    continue;
                }
                let compatible_from = kind
                    .frontmatter
                    .compatible_from
                    .unwrap_or(kind.frontmatter.revision);
                if from_revision.is_some_and(|revision| {
                    revision < compatible_from || revision > kind.frontmatter.revision
                }) {
                    report.blocked.push(KindMigrationBlock {
                        kind: Some(kind.id.clone()),
                        path: rel.display().to_string(),
                        reason: format!(
                            "schema revision {} is outside compatibility window {}..={}",
                            from_revision.expect("checked revision"),
                            compatible_from,
                            kind.frontmatter.revision
                        ),
                    });
                    continue;
                }
                mapping.insert(
                    revision_key,
                    serde_yaml::to_value(kind.frontmatter.revision)?,
                );
                let validation = validate_kind_frontmatter(kind, &mapping)
                    .into_iter()
                    .filter(|issue| issue.category != KindIssueCategory::SchemaRevision)
                    .collect::<Vec<_>>();
                if !validation.is_empty() {
                    report.blocked.push(KindMigrationBlock {
                        kind: Some(kind.id.clone()),
                        path: rel.display().to_string(),
                        reason: validation
                            .iter()
                            .map(|issue| issue.message.as_str())
                            .collect::<Vec<_>>()
                            .join("; "),
                    });
                    continue;
                }
                let updated = replace_frontmatter_revision(
                    &text,
                    &kind.frontmatter.revision_field,
                    kind.frontmatter.revision,
                )?;
                if pending.insert(rel.clone(), updated).is_some() {
                    report.blocked.push(KindMigrationBlock {
                        kind: Some(kind.id.clone()),
                        path: rel.display().to_string(),
                        reason: "document is selected by more than one Document Kind".to_owned(),
                    });
                    continue;
                }
                report.schema_changes.push(KindMigrationChange {
                    kind: kind.id.clone(),
                    path: rel.display().to_string(),
                    from_revision,
                    to_revision: kind.frontmatter.revision,
                });
            }
        }
    }
    report
        .schema_changes
        .sort_by(|left, right| left.path.cmp(&right.path));
    report
        .template_changes
        .sort_by(|left, right| left.profile.cmp(&right.profile));
    report
        .blocked
        .sort_by(|left, right| (&left.path, &left.reason).cmp(&(&right.path, &right.reason)));

    if apply && report.blocked.is_empty() {
        for (rel, content) in pending {
            std::fs::write(root.join(&rel), content)
                .with_context(|| format!("failed to migrate {}", rel.display()))?;
        }
        *config = candidate;
        report.applied = !report.schema_changes.is_empty() || !report.template_changes.is_empty();
    }
    Ok(report)
}

pub fn print_text_kind_migration(report: &KindMigrationReport) {
    for change in &report.schema_changes {
        println!(
            "schema {} {}: {} -> {}",
            change.kind,
            change.path,
            change
                .from_revision
                .map(|value| value.to_string())
                .unwrap_or_else(|| "missing".to_owned()),
            change.to_revision
        );
    }
    for change in &report.template_changes {
        println!(
            "template {} / {}: {} -> {}",
            change.template,
            change.profile,
            change
                .from_revision
                .map(|value| value.to_string())
                .unwrap_or_else(|| "missing".to_owned()),
            change.to_revision
        );
    }
    for blocked in &report.blocked {
        println!("blocked {}: {}", blocked.path, blocked.reason);
    }
    if report.is_clean() {
        println!("Document Kind schemas and template pins are current.");
    } else if report.applied {
        println!("Applied all compatible Document Kind and template migrations.");
    }
}

pub fn print_json_kind_migration(report: &KindMigrationReport) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(report)?);
    Ok(())
}

fn replace_frontmatter_revision(text: &str, field: &str, revision: u64) -> Result<String> {
    let Some(rest) = text.strip_prefix("---\n") else {
        bail!("document requires YAML frontmatter");
    };
    let Some((yaml, body)) = rest.split_once("\n---") else {
        bail!("document has unterminated YAML frontmatter");
    };
    let mut found = false;
    let mut lines = yaml
        .lines()
        .map(|line| {
            if line.starts_with(&format!("{field}:")) {
                found = true;
                format!("{field}: {revision}")
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>();
    if !found {
        lines.insert(0, format!("{field}: {revision}"));
    }
    Ok(format!("---\n{}\n---{}", lines.join("\n"), body))
}

fn compile_globs(patterns: &[String]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        builder.add(Glob::new(pattern)?);
    }
    Ok(builder.build()?)
}

fn first_profile_owner(config: &Config, rel: &Path) -> Result<Option<String>> {
    let filename = rel
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    for profile in &config.document_contracts.profiles {
        let paths = compile_globs(&profile.matcher.paths)?;
        let path_matches = profile.matcher.paths.is_empty() || paths.is_match(rel);
        let filename_matches = profile.matcher.filenames.is_empty()
            || profile
                .matcher
                .filenames
                .iter()
                .any(|pattern| Regex::new(pattern).is_ok_and(|regex| regex.is_match(filename)));
        if path_matches && filename_matches {
            return Ok(Some(profile.id.clone()));
        }
    }
    Ok(None)
}
