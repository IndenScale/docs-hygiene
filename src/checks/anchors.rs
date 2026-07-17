fn validate_edge_anchor(
    root: &Path,
    config: &Config,
    edge: &GovernanceEdge,
    target: &SemanticTarget,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(anchor) = &edge.content_anchor else {
        return;
    };
    match anchor.scope {
        ContentAnchorScope::File => validate_file_anchor(root, edge, target, anchor, diagnostics),
        ContentAnchorScope::Block => {
            validate_block_anchor(root, edge, target, anchor, diagnostics)
        }
        ContentAnchorScope::Commit => {
            validate_commit_anchor(root, config, edge, target, anchor, diagnostics)
        }
    }
}

fn validate_file_anchor(
    root: &Path,
    edge: &GovernanceEdge,
    target: &SemanticTarget,
    anchor: &ContentAnchor,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(target_bytes) = read_anchor_target(root, edge, target, diagnostics) else {
        return;
    };
    let actual_hash = format!("{:x}", Sha256::digest(target_bytes));
    if actual_hash != anchor.digest {
        push_anchor_diagnostic(
            edge,
            target,
            diagnostics,
            format!(
                "Wiki Link target '{}' changed: expected sha256:{}, actual sha256:{actual_hash}.",
                edge.target, anchor.digest
            ),
        );
    }
}

fn validate_block_anchor(
    root: &Path,
    edge: &GovernanceEdge,
    target: &SemanticTarget,
    anchor: &ContentAnchor,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(locator) = anchor.locator.as_deref() else {
        return;
    };
    let Ok(text) = std::fs::read_to_string(root.join(&target.path)) else {
        return;
    };
    let Some(block) = markdown_heading_block(&text, locator) else {
        return;
    };
    let actual_hash = format!("{:x}", Sha256::digest(block));
    if actual_hash != anchor.digest {
        push_anchor_diagnostic(
            edge,
            target,
            diagnostics,
            format!(
                "Pinned block '#{locator}' in target '{}' changed: expected sha256:{}, actual sha256:{actual_hash}.",
                edge.target, anchor.digest
            ),
        );
    }
}

fn validate_commit_anchor(
    root: &Path,
    config: &Config,
    edge: &GovernanceEdge,
    target: &SemanticTarget,
    anchor: &ContentAnchor,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !config.governance.content_anchors.verify_git_commits {
        push_anchor_diagnostic(
            edge,
            target,
            diagnostics,
            "Commit anchor verification requires governance.contentAnchors.verifyGitCommits: true."
                .to_owned(),
        );
        return;
    }
    let commit_object = format!("{}^{{commit}}", anchor.digest);
    let commit_status = Command::new("git")
        .args(["-C"])
        .arg(root)
        .args(["cat-file", "-e"])
        .arg(&commit_object)
        .output();
    match commit_status {
        Ok(output) if output.status.success() => {}
        Ok(output) => {
            let detail = String::from_utf8_lossy(&output.stderr);
            push_anchor_diagnostic(
                edge,
                target,
                diagnostics,
                format!(
                    "Commit anchor '{}' is not a resolvable Git commit: {}.",
                    anchor.digest,
                    detail.trim()
                ),
            );
            return;
        }
        Err(error) => {
            push_anchor_diagnostic(
                edge,
                target,
                diagnostics,
                format!("Commit anchor verification cannot run Git: {error}."),
            );
            return;
        }
    }
    let object = format!("{}:{}", anchor.digest, target.path);
    let output = Command::new("git")
        .args(["-C"])
        .arg(root)
        .args(["cat-file", "blob"])
        .arg(&object)
        .output();
    let pinned_bytes = match output {
        Ok(output) if output.status.success() => output.stdout,
        Ok(output) => {
            let detail = String::from_utf8_lossy(&output.stderr);
            push_anchor_diagnostic(
                edge,
                target,
                diagnostics,
                format!(
                    "Commit anchor '{}' cannot resolve governed target '{}': {}.",
                    anchor.digest,
                    target.path,
                    detail.trim()
                ),
            );
            return;
        }
        Err(error) => {
            push_anchor_diagnostic(
                edge,
                target,
                diagnostics,
                format!("Commit anchor verification cannot run Git: {error}."),
            );
            return;
        }
    };
    let Some(current_bytes) = read_anchor_target(root, edge, target, diagnostics) else {
        return;
    };
    if pinned_bytes != current_bytes {
        push_anchor_diagnostic(
            edge,
            target,
            diagnostics,
            format!(
                "Commit-pinned target '{}' differs from Git object '{}:{}'.",
                edge.target, anchor.digest, target.path
            ),
        );
    }
}

fn read_anchor_target(
    root: &Path,
    edge: &GovernanceEdge,
    target: &SemanticTarget,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<Vec<u8>> {
    match std::fs::read(root.join(&target.path)) {
        Ok(bytes) => Some(bytes),
        Err(error) => {
            push_anchor_diagnostic(
                edge,
                target,
                diagnostics,
                format!("Content-hash target '{}' cannot be read: {error}.", edge.target),
            );
            None
        }
    }
}

fn push_anchor_diagnostic(
    edge: &GovernanceEdge,
    target: &SemanticTarget,
    diagnostics: &mut Vec<Diagnostic>,
    message: String,
) {
    let mut diagnostic = Diagnostic::new(
        "DH_REFERENCE_001",
        Severity::Error,
        edge.source_location.path.clone(),
        message,
    )
    .with_related(RelatedInformation::new(
        target.path.clone(),
        "Pinned Library content is here.",
    ));
    if let Some(line) = edge.source_location.line {
        diagnostic = diagnostic.at_line(line);
    }
    diagnostics.push(diagnostic);
}
