enum PinState {
    Current,
    Stale { actual: String },
    Invalid(String),
}

fn critical_pin_state(
    root: &Path,
    config: &Config,
    pin: &GovernanceEdge,
    target: &GovernanceNode,
) -> PinState {
    let Some(anchor) = &pin.content_anchor else {
        return PinState::Invalid("normalized pin lacks an anchor".to_owned());
    };
    match anchor.scope {
        ContentAnchorScope::File => match std::fs::read(root.join(&target.location.path)) {
            Ok(bytes) => compare_sha256(anchor, &bytes),
            Err(error) => PinState::Invalid(format!("target cannot be read: {error}")),
        },
        ContentAnchorScope::Block => {
            let Some(locator) = anchor.locator.as_deref() else {
                return PinState::Invalid("block anchor lacks locator".to_owned());
            };
            let text = match std::fs::read_to_string(root.join(&target.location.path)) {
                Ok(text) => text,
                Err(error) => {
                    return PinState::Invalid(format!("target cannot be read: {error}"));
                }
            };
            match markdown_heading_block(&text, locator) {
                Some(bytes) => compare_sha256(anchor, bytes),
                None => PinState::Invalid(format!("block selector '#{locator}' does not resolve")),
            }
        }
        ContentAnchorScope::Repo => {
            if !config.governance.content_anchors.verify_git_commits {
                return PinState::Invalid("repo anchor verification is disabled".to_owned());
            }
            match crate::repository_anchor::verify_repository_anchor(root, &anchor.digest) {
                crate::repository_anchor::RepositoryAnchorState::Current => PinState::Current,
                crate::repository_anchor::RepositoryAnchorState::Stale => PinState::Stale {
                    actual: "tracked-repository-state".to_owned(),
                },
                crate::repository_anchor::RepositoryAnchorState::Invalid(error) => {
                    PinState::Invalid(error)
                }
            }
        }
    }
}

fn compare_sha256(anchor: &ContentAnchor, bytes: &[u8]) -> PinState {
    let actual = format!("{:x}", Sha256::digest(bytes));
    if actual == anchor.digest {
        PinState::Current
    } else {
        PinState::Stale { actual }
    }
}

fn pin_is_fresh(pin: &GovernanceEdge, max_age_days: u64) -> bool {
    let Some(anchor) = &pin.content_anchor else {
        return false;
    };
    let Some(updated) = anchor.updated_at.as_deref().and_then(parse_iso_date) else {
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
    let Some(today) = today_utc() else {
        return false;
    };
    let age = ownership_days_from_civil(today).saturating_sub(ownership_days_from_civil(updated));
    age >= 0 && u64::try_from(age).is_ok_and(|age| age <= max_age_days)
}
