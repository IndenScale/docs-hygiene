use std::path::Path;
use std::process::Command;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RepositoryAnchorState {
    Current,
    Stale,
    Invalid(String),
}

pub(crate) fn verify_repository_anchor(root: &Path, commit: &str) -> RepositoryAnchorState {
    let commit_object = format!("{commit}^{{commit}}");
    let commit_status = Command::new("git")
        .args(["-C"])
        .arg(root)
        .args(["cat-file", "-e"])
        .arg(&commit_object)
        .output();
    match commit_status {
        Ok(output) if output.status.success() => {}
        Ok(output) => {
            return RepositoryAnchorState::Invalid(
                String::from_utf8_lossy(&output.stderr).trim().to_owned(),
            );
        }
        Err(error) => {
            return RepositoryAnchorState::Invalid(format!("Git cannot run: {error}"));
        }
    }

    let comparison = Command::new("git")
        .args(["-C"])
        .arg(root)
        .args([
            "diff",
            "--quiet",
            "--no-ext-diff",
            "--ignore-submodules=none",
            commit,
            "--",
        ])
        .output();
    match comparison {
        Ok(output) if output.status.success() => RepositoryAnchorState::Current,
        Ok(output) if output.status.code() == Some(1) => RepositoryAnchorState::Stale,
        Ok(output) => RepositoryAnchorState::Invalid(
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ),
        Err(error) => RepositoryAnchorState::Invalid(format!("Git cannot run: {error}")),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;

    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    use tempfile::tempdir;

    use super::*;

    fn git(root: &Path, args: &[&str]) {
        assert!(
            Command::new("git")
                .args(args)
                .current_dir(root)
                .status()
                .unwrap()
                .success()
        );
    }

    #[test]
    fn repository_anchor_covers_all_tracked_state_and_ignores_untracked_files() {
        let temp = tempdir().unwrap();
        git(temp.path(), &["init", "-q"]);
        git(
            temp.path(),
            &["config", "user.email", "docs-hygiene@example.invalid"],
        );
        git(temp.path(), &["config", "user.name", "Docs Hygiene Test"]);
        fs::write(temp.path().join("one.md"), "one\n").unwrap();
        fs::write(temp.path().join("two.md"), "two\n").unwrap();
        git(temp.path(), &["add", "one.md", "two.md"]);
        git(temp.path(), &["commit", "-q", "-m", "baseline"]);
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        let commit = String::from_utf8(output.stdout).unwrap();
        let commit = commit.trim();

        assert_eq!(
            verify_repository_anchor(temp.path(), commit),
            RepositoryAnchorState::Current
        );
        fs::write(temp.path().join("untracked.md"), "ignored\n").unwrap();
        assert_eq!(
            verify_repository_anchor(temp.path(), commit),
            RepositoryAnchorState::Current
        );

        git(temp.path(), &["add", "untracked.md"]);
        assert_eq!(
            verify_repository_anchor(temp.path(), commit),
            RepositoryAnchorState::Stale
        );
        git(temp.path(), &["reset", "-q", "--", "untracked.md"]);
        assert_eq!(
            verify_repository_anchor(temp.path(), commit),
            RepositoryAnchorState::Current
        );

        fs::remove_file(temp.path().join("two.md")).unwrap();
        assert_eq!(
            verify_repository_anchor(temp.path(), commit),
            RepositoryAnchorState::Stale
        );
        fs::write(temp.path().join("two.md"), "two\n").unwrap();
        assert_eq!(
            verify_repository_anchor(temp.path(), commit),
            RepositoryAnchorState::Current
        );

        #[cfg(unix)]
        {
            let tracked = temp.path().join("two.md");
            let original_mode = fs::metadata(&tracked).unwrap().permissions().mode();
            fs::set_permissions(&tracked, fs::Permissions::from_mode(original_mode | 0o100))
                .unwrap();
            assert_eq!(
                verify_repository_anchor(temp.path(), commit),
                RepositoryAnchorState::Stale
            );
            fs::set_permissions(&tracked, fs::Permissions::from_mode(original_mode)).unwrap();
            assert_eq!(
                verify_repository_anchor(temp.path(), commit),
                RepositoryAnchorState::Current
            );
        }

        fs::write(temp.path().join("two.md"), "changed\n").unwrap();
        assert_eq!(
            verify_repository_anchor(temp.path(), commit),
            RepositoryAnchorState::Stale
        );
    }
}
