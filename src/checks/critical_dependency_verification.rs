enum PinState {
    Current,
    Stale { actual: String },
    Invalid(String),
}

fn critical_scope(scope: ContentAnchorScope) -> CriticalPinScope {
    match scope {
        ContentAnchorScope::File => CriticalPinScope::File,
        ContentAnchorScope::Commit => CriticalPinScope::Commit,
        ContentAnchorScope::Block => CriticalPinScope::Block,
    }
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
        ContentAnchorScope::Commit => {
            if !config.governance.content_anchors.verify_git_commits {
                return PinState::Invalid("commit verification is disabled".to_owned());
            }
            let object = format!("{}:{}", anchor.digest, target.location.path);
            match Command::new("git")
                .args(["-C"])
                .arg(root)
                .args(["cat-file", "blob"])
                .arg(&object)
                .output()
            {
                Ok(output) if output.status.success() => {
                    match std::fs::read(root.join(&target.location.path)) {
                        Ok(current) if current == output.stdout => PinState::Current,
                        Ok(_) => PinState::Stale {
                            actual: "working-tree-content".to_owned(),
                        },
                        Err(error) => PinState::Invalid(format!("target cannot be read: {error}")),
                    }
                }
                Ok(output) => PinState::Invalid(
                    String::from_utf8_lossy(&output.stderr).trim().to_owned(),
                ),
                Err(error) => PinState::Invalid(format!("Git cannot run: {error}")),
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
    let age = civil_days(today).saturating_sub(civil_days(updated));
    age >= 0 && u64::try_from(age).is_ok_and(|age| age <= max_age_days)
}

fn civil_days((year, month, day): (i32, u32, u32)) -> i64 {
    let year = i64::from(year) - i64::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let month = i64::from(month);
    let day_of_year = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5
        + i64::from(day)
        - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}
