use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result, bail};

pub(crate) fn is_safe_relative(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
}

pub(crate) fn ensure_safe_relative(path: &Path) -> Result<()> {
    if !is_safe_relative(path) {
        bail!("unsafe project-relative path '{}'", path.display());
    }
    Ok(())
}

pub(crate) fn write_batch_atomically<T: AsRef<[u8]>>(
    root: &Path,
    pending: &BTreeMap<PathBuf, T>,
) -> Result<()> {
    let process = std::process::id();
    let mut prepared = Vec::new();
    for (index, (rel, content)) in pending.iter().enumerate() {
        ensure_safe_relative(rel)?;
        let destination = root.join(rel);
        let parent = destination
            .parent()
            .ok_or_else(|| anyhow::anyhow!("{} has no parent", rel.display()))?;
        std::fs::create_dir_all(parent)?;
        let temporary = parent.join(format!(".docs-hygiene-atomic-{process}-{index}.tmp"));
        if let Err(error) = std::fs::write(&temporary, content) {
            remove_temporaries(&prepared);
            return Err(error).with_context(|| format!("failed to prepare {}", rel.display()));
        }
        let original = match std::fs::read(&destination) {
            Ok(bytes) => Some(bytes),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => {
                let _ = std::fs::remove_file(&temporary);
                remove_temporaries(&prepared);
                return Err(error).with_context(|| format!("failed to snapshot {}", rel.display()));
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
            remove_temporaries(&prepared[committed..]);
            return Err(error).context("atomic project update failed and was rolled back");
        }
    }
    Ok(())
}

fn remove_temporaries(prepared: &[(PathBuf, PathBuf, Option<Vec<u8>>)]) {
    for (_, temporary, _) in prepared {
        let _ = std::fs::remove_file(temporary);
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn atomic_batch_writes_all_targets_and_rejects_escape_paths() {
        let root = tempdir().unwrap();
        let pending = BTreeMap::from([
            (PathBuf::from("one.txt"), "one".to_owned()),
            (PathBuf::from("nested/two.txt"), "two".to_owned()),
        ]);
        write_batch_atomically(root.path(), &pending).unwrap();
        assert_eq!(
            std::fs::read_to_string(root.path().join("one.txt")).unwrap(),
            "one"
        );
        assert_eq!(
            std::fs::read_to_string(root.path().join("nested/two.txt")).unwrap(),
            "two"
        );

        let escaped = BTreeMap::from([(PathBuf::from("../escape.txt"), "bad".to_owned())]);
        assert!(write_batch_atomically(root.path(), &escaped).is_err());
        assert!(!root.path().parent().unwrap().join("escape.txt").exists());
    }
}
