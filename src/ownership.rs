use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::Serialize;
use serde_yaml::{Mapping, Value};

use crate::config::{Config, GovernancePrincipalKind, GovernancePrincipalStatus};
use crate::run_checks;

mod date;
mod output;

use date::{format_date, parse_date, utc_today};

pub use output::{print_json_review_reset, print_text_review_reset};

#[derive(Clone, Debug)]
pub struct ReviewResetRequest {
    pub identity: String,
    pub actor: String,
    pub reason: String,
    pub review_by: String,
    pub apply: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewResetChange {
    pub identity: String,
    pub path: String,
    pub previous_review_by: Option<String>,
    pub review_by: String,
    pub reset_at: String,
    pub reset_by: String,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewResetBlock {
    pub identity: String,
    pub path: String,
    pub reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewResetReport {
    pub schema_version: &'static str,
    pub changes: Vec<ReviewResetChange>,
    pub blocked: Vec<ReviewResetBlock>,
    pub audit_log: String,
    pub applied: bool,
}

pub fn reset_governed_review(
    root: &Path,
    config: &Config,
    request: &ReviewResetRequest,
) -> Result<ReviewResetReport> {
    let mut report = ReviewResetReport {
        schema_version: "docs-hygiene.review-reset.v1",
        changes: Vec::new(),
        blocked: Vec::new(),
        audit_log: config
            .governance
            .ownership
            .reset_audit_log
            .display()
            .to_string(),
        applied: false,
    };
    if !config.governance.ownership.is_configured() {
        block(
            &mut report,
            request,
            "docs-hygiene.yml",
            "ownership governance is not configured",
        );
        return Ok(report);
    }
    if request.identity.trim().is_empty()
        || request.actor.trim().is_empty()
        || request.reason.trim().is_empty()
    {
        block(
            &mut report,
            request,
            "docs-hygiene.yml",
            "review reset requires non-empty identity, actor, and reason",
        );
        return Ok(report);
    }
    if !valid_principal_directory(config) {
        block(
            &mut report,
            request,
            "docs-hygiene.yml",
            "principal directory must be valid and unique before a review reset",
        );
        return Ok(report);
    }
    let actor_matches = config
        .governance
        .ownership
        .principals
        .iter()
        .filter(|principal| {
            principal.id == request.actor
                && principal.kind == GovernancePrincipalKind::Person
                && principal.status == GovernancePrincipalStatus::Active
        })
        .count();
    if actor_matches != 1 {
        block(
            &mut report,
            request,
            "docs-hygiene.yml",
            "review reset actor must resolve to an active person principal",
        );
        return Ok(report);
    }
    let today = utc_today();
    let Some(deadline) = parse_date(&request.review_by) else {
        block(
            &mut report,
            request,
            "docs-hygiene.yml",
            "new review deadline must be a valid YYYY-MM-DD date",
        );
        return Ok(report);
    };
    if deadline <= today {
        block(
            &mut report,
            request,
            "docs-hygiene.yml",
            "new review deadline must be later than the current UTC date",
        );
        return Ok(report);
    }

    let checks = run_checks(root, config)?;
    let matches = checks
        .ownership
        .identities
        .iter()
        .filter(|identity| identity.identity == request.identity)
        .collect::<Vec<_>>();
    let [target] = matches.as_slice() else {
        block(
            &mut report,
            request,
            "docs-hygiene.yml",
            "target must resolve uniquely to one baselined or current governed identity",
        );
        return Ok(report);
    };
    let rel = PathBuf::from(&target.path);
    ensure_safe_relative(&rel)?;
    let content = std::fs::read_to_string(root.join(&rel))?;
    let document = match ReviewDocument::parse(&target.path, &content) {
        Ok(document) => document,
        Err(error) => {
            block(
                &mut report,
                request,
                &target.path,
                &format!("target review metadata cannot be edited: {error}"),
            );
            return Ok(report);
        }
    };
    if document.id() != Some(request.identity.as_str()) {
        block(
            &mut report,
            request,
            &target.path,
            "target file identity changed while planning the reset",
        );
        return Ok(report);
    }
    let previous_review_by = document.review_by();
    if previous_review_by
        .as_deref()
        .and_then(parse_date)
        .is_some_and(|previous| deadline <= previous)
    {
        block(
            &mut report,
            request,
            &target.path,
            "new review deadline must advance the existing deadline",
        );
        return Ok(report);
    }
    let reset_at = format_date(today);
    let change = ReviewResetChange {
        identity: request.identity.clone(),
        path: target.path.clone(),
        previous_review_by,
        review_by: request.review_by.clone(),
        reset_at,
        reset_by: request.actor.clone(),
        reason: request.reason.clone(),
    };
    let updated = match document.with_reset(&change) {
        Ok(updated) => updated,
        Err(error) => {
            block(
                &mut report,
                request,
                &target.path,
                &format!("target review metadata cannot be updated: {error}"),
            );
            return Ok(report);
        }
    };
    report.changes.push(change.clone());
    if request.apply {
        apply_review_reset(root, config, &rel, updated, &change)?;
        report.applied = true;
    }
    Ok(report)
}

enum ReviewDocument {
    Yaml(Mapping),
    Markdown { frontmatter: Mapping, body: String },
}

impl ReviewDocument {
    fn parse(path: &str, content: &str) -> Result<Self> {
        if Path::new(path).extension().and_then(|value| value.to_str()) == Some("md") {
            let Some(rest) = content.strip_prefix("---\n") else {
                bail!("{path} requires YAML frontmatter");
            };
            let Some((yaml, body)) = rest.split_once("\n---") else {
                bail!("{path} has unterminated YAML frontmatter");
            };
            Ok(Self::Markdown {
                frontmatter: yaml_mapping(yaml, path)?,
                body: body.to_owned(),
            })
        } else {
            Ok(Self::Yaml(yaml_mapping(content, path)?))
        }
    }

    fn mapping(&self) -> &Mapping {
        match self {
            Self::Yaml(mapping)
            | Self::Markdown {
                frontmatter: mapping,
                ..
            } => mapping,
        }
    }

    fn id(&self) -> Option<&str> {
        yaml_string(self.mapping(), "id")
    }

    fn review_by(&self) -> Option<String> {
        self.mapping()
            .get(Value::String("review".to_owned()))
            .and_then(Value::as_mapping)
            .and_then(|review| yaml_string(review, "reviewBy"))
            .map(str::to_owned)
    }

    fn with_reset(mut self, change: &ReviewResetChange) -> Result<String> {
        let mapping = match &mut self {
            Self::Yaml(mapping)
            | Self::Markdown {
                frontmatter: mapping,
                ..
            } => mapping,
        };
        let review = mapping
            .entry(Value::String("review".to_owned()))
            .or_insert_with(|| Value::Mapping(Mapping::new()));
        let Some(review) = review.as_mapping_mut() else {
            bail!("{} review metadata must be a mapping", change.path);
        };
        set_yaml(review, "reviewBy", Value::String(change.review_by.clone()));
        let mut reset = Mapping::new();
        set_yaml(&mut reset, "at", Value::String(change.reset_at.clone()));
        set_yaml(&mut reset, "by", Value::String(change.reset_by.clone()));
        set_yaml(&mut reset, "reason", Value::String(change.reason.clone()));
        set_yaml(review, "lastReset", Value::Mapping(reset));
        match self {
            Self::Yaml(mapping) => Ok(serde_yaml::to_string(&mapping)?),
            Self::Markdown { frontmatter, body } => Ok(format!(
                "---\n{}---{}",
                serde_yaml::to_string(&frontmatter)?,
                body
            )),
        }
    }
}

fn apply_review_reset(
    root: &Path,
    config: &Config,
    target: &Path,
    updated: String,
    change: &ReviewResetChange,
) -> Result<()> {
    let audit = &config.governance.ownership.reset_audit_log;
    ensure_safe_relative(audit)?;
    if audit == target {
        bail!("review reset audit log must not overlap the governed target");
    }
    let audit_path = root.join(audit);
    let mut audit_content = match std::fs::read_to_string(&audit_path) {
        Ok(content) => content,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(error) => return Err(error).context("failed to read review reset audit log"),
    };
    audit_content.push_str(&serde_json::to_string(change)?);
    audit_content.push('\n');
    let pending = BTreeMap::from([
        (target.to_path_buf(), updated),
        (audit.to_path_buf(), audit_content),
    ]);
    write_batch_atomically(root, &pending)
}

fn write_batch_atomically(root: &Path, pending: &BTreeMap<PathBuf, String>) -> Result<()> {
    let process = std::process::id();
    let mut prepared = Vec::new();
    for (index, (rel, content)) in pending.iter().enumerate() {
        let destination = root.join(rel);
        let parent = destination
            .parent()
            .ok_or_else(|| anyhow::anyhow!("{} has no parent", rel.display()))?;
        std::fs::create_dir_all(parent)?;
        let temporary = parent.join(format!(".docs-hygiene-review-{process}-{index}.tmp"));
        if let Err(error) = std::fs::write(&temporary, content) {
            for (_, temporary, _) in &prepared {
                let _ = std::fs::remove_file(temporary);
            }
            return Err(error).context("failed to prepare review reset target");
        }
        let original = match std::fs::read(&destination) {
            Ok(bytes) => Some(bytes),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => {
                let _ = std::fs::remove_file(&temporary);
                for (_, temporary, _) in &prepared {
                    let _ = std::fs::remove_file(temporary);
                }
                return Err(error).context("failed to snapshot review reset target");
            }
        };
        prepared.push((destination, temporary, original));
    }
    for (committed, (destination, temporary, _)) in prepared.iter().enumerate() {
        if let Err(error) = std::fs::rename(temporary, destination) {
            for (destination, _, original) in prepared.iter().take(committed).rev() {
                match original {
                    Some(bytes) => {
                        let _ = std::fs::write(destination, bytes);
                    }
                    None => {
                        let _ = std::fs::remove_file(destination);
                    }
                }
            }
            for (_, temporary, _) in prepared.iter().skip(committed) {
                let _ = std::fs::remove_file(temporary);
            }
            return Err(error).context("review reset commit failed and was rolled back");
        }
    }
    Ok(())
}

fn yaml_mapping(yaml: &str, path: &str) -> Result<Mapping> {
    match serde_yaml::from_str::<Value>(yaml)? {
        Value::Mapping(mapping) => Ok(mapping),
        _ => bail!("{path} YAML must be a mapping"),
    }
}

fn yaml_string<'a>(mapping: &'a Mapping, key: &str) -> Option<&'a str> {
    mapping
        .get(Value::String(key.to_owned()))
        .and_then(Value::as_str)
}

fn set_yaml(mapping: &mut Mapping, key: &str, value: Value) {
    mapping.insert(Value::String(key.to_owned()), value);
}

fn ensure_safe_relative(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty()
        || path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        bail!("unsafe project-relative path '{}'", path.display());
    }
    Ok(())
}

fn valid_principal_directory(config: &Config) -> bool {
    let principals = &config.governance.ownership.principals;
    let index = principals
        .iter()
        .map(|principal| (principal.id.as_str(), principal))
        .collect::<BTreeMap<_, _>>();
    if index.len() != principals.len() {
        return false;
    }
    principals.iter().all(|principal| match principal.kind {
        GovernancePrincipalKind::Person => {
            valid_principal_id(&principal.id, "person:") && principal.members.is_empty()
        }
        GovernancePrincipalKind::Group => {
            valid_principal_id(&principal.id, "group:")
                && !principal.members.is_empty()
                && principal
                    .members
                    .iter()
                    .collect::<std::collections::BTreeSet<_>>()
                    .len()
                    == principal.members.len()
                && principal.members.iter().all(|member| {
                    index
                        .get(member.as_str())
                        .is_some_and(|member| member.kind == GovernancePrincipalKind::Person)
                })
        }
    })
}

fn valid_principal_id(id: &str, prefix: &str) -> bool {
    id.strip_prefix(prefix).is_some_and(|suffix| {
        !suffix.is_empty()
            && suffix
                .chars()
                .all(|value| value.is_ascii_alphanumeric() || matches!(value, '.' | '_' | '-'))
    })
}

fn block(report: &mut ReviewResetReport, request: &ReviewResetRequest, path: &str, reason: &str) {
    report.blocked.push(ReviewResetBlock {
        identity: request.identity.clone(),
        path: path.to_owned(),
        reason: reason.to_owned(),
    });
}
