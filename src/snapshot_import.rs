use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::{
    ContentAnchorScope, PORTABLE_SNAPSHOT_SCHEMA_VERSION, PortableSnapshotManifest,
    PortableSnapshotStatus,
};

pub const SNAPSHOT_IMPORT_SCHEMA_VERSION: &str = "docs-hygiene.snapshot-import.v1";

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotImportChange {
    pub target: String,
    pub source_path: String,
    pub payload_path: String,
    pub scope: ContentAnchorScope,
    pub locator: Option<String>,
    pub digest: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotImportBlock {
    pub target: Option<String>,
    pub path: String,
    pub reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotImportReport {
    pub schema_version: &'static str,
    pub snapshot: Option<String>,
    pub changes: Vec<SnapshotImportChange>,
    pub blocked: Vec<SnapshotImportBlock>,
    pub applied: bool,
}

pub fn import_portable_snapshot(
    root: &Path,
    manifest_rel: &Path,
    source: &Path,
    apply: bool,
) -> Result<SnapshotImportReport> {
    let mut report = SnapshotImportReport {
        schema_version: SNAPSHOT_IMPORT_SCHEMA_VERSION,
        snapshot: None,
        changes: Vec::new(),
        blocked: Vec::new(),
        applied: false,
    };
    if !safe_relative(manifest_rel) {
        report.blocked.push(SnapshotImportBlock {
            target: None,
            path: manifest_rel.display().to_string(),
            reason: "snapshot manifest must be a safe project-relative path".to_owned(),
        });
        return Ok(report);
    }
    let manifest_text = std::fs::read_to_string(root.join(manifest_rel))?;
    let manifest: PortableSnapshotManifest = serde_yaml::from_str(&manifest_text)?;
    report.snapshot = Some(manifest.id.clone());
    if manifest.schema_version != PORTABLE_SNAPSHOT_SCHEMA_VERSION
        || manifest.status != PortableSnapshotStatus::Active
        || !valid_commit_oid(&manifest.commit)
    {
        report.blocked.push(SnapshotImportBlock {
            target: None,
            path: manifest_rel.display().to_string(),
            reason: format!(
                "snapshot import requires schemaVersion '{}' and active status",
                PORTABLE_SNAPSHOT_SCHEMA_VERSION
            ),
        });
        return Ok(report);
    }
    if !git_commit_exists(source, &manifest.commit)? {
        report.blocked.push(SnapshotImportBlock {
            target: None,
            path: manifest_rel.display().to_string(),
            reason: format!(
                "commit '{}' is not available in the explicit local source checkout",
                manifest.commit
            ),
        });
        return Ok(report);
    }
    let manifest_dir = manifest_rel.parent().unwrap_or_else(|| Path::new(""));
    let mut pending = BTreeMap::<PathBuf, Vec<u8>>::new();
    for entry in &manifest.entries {
        if !safe_snapshot_path(&entry.path) || !safe_snapshot_path(&entry.payload) {
            report.blocked.push(SnapshotImportBlock {
                target: Some(entry.target.clone()),
                path: entry.path.clone(),
                reason: "snapshot source and payload paths must be safe and relative".to_owned(),
            });
            continue;
        }
        let blob = match git_blob(source, &manifest.commit, &entry.path) {
            Ok(blob) => blob,
            Err(error) => {
                report.blocked.push(SnapshotImportBlock {
                    target: Some(entry.target.clone()),
                    path: entry.path.clone(),
                    reason: error.to_string(),
                });
                continue;
            }
        };
        let pinned = match entry.scope {
            ContentAnchorScope::File => Some(blob.as_slice()),
            ContentAnchorScope::Block => std::str::from_utf8(&blob).ok().and_then(|text| {
                entry
                    .locator
                    .as_deref()
                    .and_then(|locator| markdown_heading_block(text, locator))
            }),
            ContentAnchorScope::Commit => None,
        };
        let Some(pinned) = pinned else {
            report.blocked.push(SnapshotImportBlock {
                target: Some(entry.target.clone()),
                path: entry.path.clone(),
                reason: "snapshot entry requires file scope or a uniquely resolved UTF-8 block"
                    .to_owned(),
            });
            continue;
        };
        let actual = format!("{:x}", Sha256::digest(pinned));
        if actual != entry.digest.to_ascii_lowercase() {
            report.blocked.push(SnapshotImportBlock {
                target: Some(entry.target.clone()),
                path: entry.path.clone(),
                reason: format!(
                    "declared digest '{}' does not match commit content '{}'",
                    entry.digest, actual
                ),
            });
            continue;
        }
        let payload = manifest_dir.join(Path::new(&entry.payload));
        if !safe_relative(&payload) || pending.insert(payload.clone(), blob.clone()).is_some() {
            report.blocked.push(SnapshotImportBlock {
                target: Some(entry.target.clone()),
                path: payload.display().to_string(),
                reason: "snapshot payload destination is unsafe or duplicated".to_owned(),
            });
            continue;
        }
        if std::fs::read(root.join(&payload)).ok().as_deref() != Some(blob.as_slice()) {
            report.changes.push(SnapshotImportChange {
                target: entry.target.clone(),
                source_path: entry.path.clone(),
                payload_path: payload.display().to_string(),
                scope: entry.scope,
                locator: entry.locator.clone(),
                digest: actual,
            });
        }
    }
    report.changes.sort_by(|left, right| {
        (&left.target, &left.payload_path).cmp(&(&right.target, &right.payload_path))
    });
    report
        .blocked
        .sort_by(|left, right| (&left.path, &left.reason).cmp(&(&right.path, &right.reason)));
    if apply && report.blocked.is_empty() && !report.changes.is_empty() {
        let changed = report
            .changes
            .iter()
            .map(|change| PathBuf::from(&change.payload_path))
            .collect::<Vec<_>>();
        let writes = pending
            .into_iter()
            .filter(|(path, _)| changed.contains(path))
            .collect::<BTreeMap<_, _>>();
        write_payloads_atomically(root, &writes)?;
        report.applied = true;
    }
    Ok(report)
}

fn git_commit_exists(source: &Path, commit: &str) -> Result<bool> {
    let object = format!("{commit}^{{commit}}");
    let output = Command::new("git")
        .args(["-C"])
        .arg(source)
        .args(["cat-file", "-e"])
        .arg(object)
        .output()
        .context("Git cannot verify the explicit snapshot source")?;
    Ok(output.status.success())
}

fn git_blob(source: &Path, commit: &str, path: &str) -> Result<Vec<u8>> {
    let object = format!("{commit}:{path}");
    let output = Command::new("git")
        .args(["-C"])
        .arg(source)
        .args(["cat-file", "blob"])
        .arg(&object)
        .output()
        .context("Git cannot read the explicit snapshot source")?;
    if !output.status.success() {
        bail!(
            "commit object '{}' cannot be read: {}",
            object,
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(output.stdout)
}

fn write_payloads_atomically(root: &Path, writes: &BTreeMap<PathBuf, Vec<u8>>) -> Result<()> {
    let process = std::process::id();
    let mut prepared = Vec::new();
    for (index, (rel, content)) in writes.iter().enumerate() {
        let destination = root.join(rel);
        let parent = destination
            .parent()
            .ok_or_else(|| anyhow::anyhow!("{} has no parent", rel.display()))?;
        std::fs::create_dir_all(parent)?;
        let temporary = parent.join(format!(".docs-hygiene-snapshot-{process}-{index}.tmp"));
        std::fs::write(&temporary, content)?;
        let original = match std::fs::read(&destination) {
            Ok(bytes) => Some(bytes),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(error.into()),
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
            return Err(error).context("snapshot payload import failed and was rolled back");
        }
    }
    Ok(())
}

fn safe_relative(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
}

fn valid_commit_oid(value: &str) -> bool {
    matches!(value.len(), 40 | 64) && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn safe_snapshot_path(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('/')
        && !value.contains('\\')
        && value
            .split('/')
            .all(|segment| !matches!(segment, "" | "." | ".."))
}

fn markdown_heading_block<'a>(text: &'a str, locator: &str) -> Option<&'a [u8]> {
    let mut headings = Vec::new();
    let mut offset = 0;
    let mut in_code = false;
    for segment in text.split_inclusive('\n') {
        let line = segment.trim_end_matches(['\n', '\r']);
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_code = !in_code;
            offset += segment.len();
            continue;
        }
        if !in_code {
            let level = trimmed.chars().take_while(|value| *value == '#').count();
            if (1..=6).contains(&level) && trimmed[level..].starts_with(char::is_whitespace) {
                let slug = heading_slug(trimmed[level..].trim().trim_end_matches('#').trim());
                if let Some(slug) = slug {
                    headings.push((slug, level, offset));
                }
            }
        }
        offset += segment.len();
    }
    let matches = headings
        .iter()
        .enumerate()
        .filter(|(_, (slug, _, _))| slug == locator)
        .collect::<Vec<_>>();
    let [(index, (_, level, start))] = matches.as_slice() else {
        return None;
    };
    let end = headings
        .iter()
        .skip(index + 1)
        .find(|(_, next_level, _)| next_level <= level)
        .map(|(_, _, start)| *start)
        .unwrap_or(text.len());
    Some(&text.as_bytes()[*start..end])
}

fn heading_slug(heading: &str) -> Option<String> {
    let mut slug = String::new();
    let mut separator = false;
    for value in heading.chars() {
        if value.is_ascii_alphanumeric() {
            if separator && !slug.is_empty() {
                slug.push('-');
            }
            slug.push(value.to_ascii_lowercase());
            separator = false;
        } else if !slug.is_empty() {
            separator = true;
        }
    }
    (!slug.is_empty()).then_some(slug)
}

pub fn print_text_snapshot_import(report: &SnapshotImportReport) {
    for change in &report.changes {
        println!(
            "{} {} -> {} ({})",
            change.target, change.source_path, change.payload_path, change.digest
        );
    }
    for blocked in &report.blocked {
        println!("blocked {}: {}", blocked.path, blocked.reason);
    }
    if report.changes.is_empty() && report.blocked.is_empty() {
        println!("Portable snapshot payloads are current.");
    } else if report.applied {
        println!("Imported portable snapshot payloads atomically.");
    }
}

pub fn print_json_snapshot_import(report: &SnapshotImportReport) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(report)?);
    Ok(())
}
