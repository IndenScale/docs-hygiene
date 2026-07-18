use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::markdown::heading_block as markdown_heading_block;
use crate::portable_snapshot::{safe_snapshot_path, valid_commit_oid};
use crate::project_io::{is_safe_relative, write_batch_atomically};
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
    if !is_safe_relative(manifest_rel) {
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
                "snapshot import requires schemaVersion '{PORTABLE_SNAPSHOT_SCHEMA_VERSION}' and active status"
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
            ContentAnchorScope::Repo => None,
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
        if !is_safe_relative(&payload) || pending.insert(payload.clone(), blob.clone()).is_some() {
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
        write_batch_atomically(root, &writes)?;
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
