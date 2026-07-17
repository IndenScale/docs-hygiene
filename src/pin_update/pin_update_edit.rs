use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde_yaml::{Mapping, Value};

use crate::config::Config;
use crate::project_io::{ensure_safe_relative, write_batch_atomically};
use crate::yaml::{mapping_string as yaml_string, set_mapping_value as set_yaml};
use crate::{ContentAnchorScope, PinUpdateChange};

pub(super) fn apply_pin_changes(
    root: &Path,
    config: &Config,
    changes: &[PinUpdateChange],
) -> Result<()> {
    let mut pending = BTreeMap::<PathBuf, String>::new();
    for change in changes {
        let rel = PathBuf::from(&change.source_path);
        ensure_safe_relative(&rel)?;
        let content = match pending.get(&rel) {
            Some(content) => content.clone(),
            None => std::fs::read_to_string(root.join(&rel))?,
        };
        pending.insert(rel, update_document_anchor(&content, change)?);
    }
    let audit_rel = &config.governance.pin_audit_log;
    ensure_safe_relative(audit_rel)?;
    let audit_path = root.join(audit_rel);
    let mut audit = match std::fs::read_to_string(&audit_path) {
        Ok(audit) => audit,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(error) => return Err(error).context("failed to read the Pin audit log"),
    };
    for change in changes {
        audit.push_str(&serde_json::to_string(change)?);
        audit.push('\n');
    }
    if pending.insert(audit_rel.clone(), audit).is_some() {
        bail!("pin audit log must not overlap a governed source document");
    }
    write_batch_atomically(root, &pending)
}

fn update_document_anchor(content: &str, change: &PinUpdateChange) -> Result<String> {
    let mut content = content.to_owned();
    if let Some(old) = &change.old_digest {
        let reference = match &change.selector {
            Some(selector) => format!("[[{}#{selector}", change.target),
            None => format!("[[{}", change.target),
        };
        content = content.replace(&format!("{reference}@sha256:{old}"), &reference);
    }
    let Some(rest) = content.strip_prefix("---\n") else {
        bail!("{} requires YAML frontmatter", change.source_path);
    };
    let Some((yaml, body)) = rest.split_once("\n---") else {
        bail!("{} has unterminated YAML frontmatter", change.source_path);
    };
    let mut mapping = match serde_yaml::from_str::<Value>(yaml)? {
        Value::Mapping(mapping) => mapping,
        _ => bail!("{} frontmatter must be a mapping", change.source_path),
    };
    let anchors_key = Value::String("anchors".to_owned());
    let anchors = mapping
        .entry(anchors_key)
        .or_insert_with(|| Value::Sequence(Vec::new()));
    let Some(anchors) = anchors.as_sequence_mut() else {
        bail!(
            "{} frontmatter anchors must be a sequence",
            change.source_path
        );
    };
    let mut updated_existing = false;
    if let Some(old) = change.old_digest.as_deref() {
        for anchor in anchors.iter_mut().filter_map(Value::as_mapping_mut) {
            if yaml_string(anchor, "target") == Some(change.target.as_str())
                && yaml_string(anchor, "digest") == Some(old)
            {
                write_anchor(anchor, change)?;
                updated_existing = true;
            }
        }
    }
    if !updated_existing {
        anchors.push(Value::Mapping(Mapping::new()));
        let anchor = anchors
            .last_mut()
            .and_then(Value::as_mapping_mut)
            .expect("inserted mapping");
        write_anchor(anchor, change)?;
    }
    let yaml = serde_yaml::to_string(&mapping)?;
    Ok(format!("---\n{yaml}---{body}"))
}

fn write_anchor(anchor: &mut Mapping, change: &PinUpdateChange) -> Result<()> {
    set_yaml(anchor, "target", Value::String(change.target.clone()));
    set_yaml(anchor, "algorithm", Value::String(change.algorithm.clone()));
    set_yaml(anchor, "digest", Value::String(change.new_digest.clone()));
    set_yaml(anchor, "scope", serde_yaml::to_value(change.scope)?);
    match &change.selector {
        Some(selector) if change.scope == ContentAnchorScope::Block => {
            set_yaml(anchor, "locator", Value::String(selector.clone()));
        }
        _ => {
            anchor.remove(Value::String("locator".to_owned()));
        }
    }
    set_yaml(
        anchor,
        "updatedAt",
        Value::String(change.updated_at.clone()),
    );
    set_yaml(
        anchor,
        "updatedBy",
        Value::String(change.updated_by.clone()),
    );
    set_yaml(anchor, "reason", Value::String(change.reason.clone()));
    Ok(())
}
