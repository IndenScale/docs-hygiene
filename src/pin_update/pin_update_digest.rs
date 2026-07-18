use std::path::Path;
use std::process::Command;

use anyhow::{Result, bail};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::date::{days_from_civil, format_date, parse_date, utc_today};
use crate::markdown::heading_block as markdown_heading_block;
use crate::{ContentAnchor, ContentAnchorScope, GovernanceNode};

pub(super) fn audit_metadata_current(anchor: &ContentAnchor, max_age_days: u64) -> bool {
    let Some(updated) = anchor.updated_at.as_deref().and_then(parse_date) else {
        return false;
    };
    if anchor
        .updated_by
        .as_deref()
        .is_none_or(|value| value.trim().is_empty())
        || anchor
            .reason
            .as_deref()
            .is_none_or(|value| value.trim().is_empty())
    {
        return false;
    }
    let Some(today) = utc_today() else {
        return false;
    };
    let age = days_from_civil(today).saturating_sub(days_from_civil(updated));
    age >= 0 && u64::try_from(age).is_ok_and(|age| age <= max_age_days)
}

pub(super) fn governed_pin_source(root: &Path, declared: &str) -> Result<String> {
    let rel = Path::new(declared);
    if rel.extension().and_then(|value| value.to_str()) == Some("md") {
        return Ok(declared.to_owned());
    }
    let text = std::fs::read_to_string(root.join(rel))?;
    let mapping = serde_yaml::from_str::<serde_yaml::Value>(&text)?;
    let members = mapping
        .as_mapping()
        .and_then(|mapping| mapping.get(serde_yaml::Value::String("members".to_owned())))
        .ok_or_else(|| anyhow::anyhow!("governance source '{declared}' has no content members"))?;
    let values = match members {
        serde_yaml::Value::Sequence(values) => values.iter().collect::<Vec<_>>(),
        serde_yaml::Value::Mapping(groups) => groups
            .values()
            .filter_map(serde_yaml::Value::as_sequence)
            .flatten()
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };
    let base = rel.parent().unwrap_or_else(|| Path::new(""));
    for value in values {
        let Some(member) = value.as_str() else {
            continue;
        };
        for candidate in [Path::new(member).to_path_buf(), base.join(member)] {
            let absolute = root.join(&candidate);
            if absolute.is_file()
                && candidate.extension().and_then(|value| value.to_str()) == Some("md")
            {
                return Ok(candidate.display().to_string());
            }
            if absolute.is_dir()
                && let Some(entry) = WalkDir::new(&absolute)
                    .sort_by_file_name()
                    .into_iter()
                    .filter_map(std::result::Result::ok)
                    .find(|entry| {
                        entry.file_type().is_file()
                            && entry.path().extension().and_then(|value| value.to_str())
                                == Some("md")
                    })
            {
                return Ok(entry.path().strip_prefix(root)?.display().to_string());
            }
        }
    }
    bail!("governance source '{declared}' has no Markdown content member")
}

pub(super) fn compute_pin_digest(
    root: &Path,
    target: &GovernanceNode,
    algorithm: &str,
    scope: ContentAnchorScope,
    selector: Option<&str>,
) -> Result<String> {
    let target_path = root.join(&target.location.path);
    match (algorithm, scope) {
        ("sha256", ContentAnchorScope::File) => {
            Ok(format!("{:x}", Sha256::digest(std::fs::read(target_path)?)))
        }
        ("sha256", ContentAnchorScope::Block) => {
            let text = std::fs::read_to_string(target_path)?;
            let selector = selector.ok_or_else(|| anyhow::anyhow!("block pin lacks selector"))?;
            let block = markdown_heading_block(&text, selector).ok_or_else(|| {
                anyhow::anyhow!("selector '#{selector}' does not resolve exactly once")
            })?;
            Ok(format!("{:x}", Sha256::digest(block)))
        }
        ("git", ContentAnchorScope::Repo) => {
            let output = Command::new("git")
                .args(["-C"])
                .arg(root)
                .args(["rev-parse", "HEAD"])
                .output()?;
            if !output.status.success() {
                bail!("cannot resolve HEAD for commit pin");
            }
            let digest = String::from_utf8(output.stdout)?.trim().to_owned();
            if crate::repository_anchor::verify_repository_anchor(root, &digest)
                != crate::repository_anchor::RepositoryAnchorState::Current
            {
                bail!(
                    "tracked repository state differs from HEAD and cannot be accepted as a repo pin"
                );
            }
            Ok(digest)
        }
        _ => bail!("unsupported algorithm/scope combination '{algorithm}/{scope:?}'"),
    }
}

pub(super) fn utc_today_string() -> String {
    utc_today()
        .map(format_date)
        .unwrap_or_else(|| "1970-01-01".to_owned())
}
