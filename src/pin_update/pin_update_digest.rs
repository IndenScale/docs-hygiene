use std::path::Path;
use std::process::Command;

use anyhow::{Result, bail};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

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
    let today = parse_date(&utc_today_string()).expect("generated UTC date");
    let age = days_from_civil(today).saturating_sub(days_from_civil(updated));
    age >= 0 && u64::try_from(age).is_ok_and(|age| age <= max_age_days)
}

fn parse_date(value: &str) -> Option<(i32, u32, u32)> {
    if value.len() != 10 || &value[4..5] != "-" || &value[7..8] != "-" {
        return None;
    }
    let year = value[..4].parse().ok()?;
    let month = value[5..7].parse().ok()?;
    let day = value[8..].parse().ok()?;
    if !(1..=12).contains(&month) {
        return None;
    }
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    (day >= 1 && day <= days[(month - 1) as usize]).then_some((year, month, day))
}

fn days_from_civil((year, month, day): (i32, u32, u32)) -> i64 {
    let year = i64::from(year) - i64::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let month = i64::from(month);
    let day_of_year = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + i64::from(day) - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
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
        ("git", ContentAnchorScope::Commit) => {
            let output = Command::new("git")
                .args(["-C"])
                .arg(root)
                .args(["rev-parse", "HEAD"])
                .output()?;
            if !output.status.success() {
                bail!("cannot resolve HEAD for commit pin");
            }
            let digest = String::from_utf8(output.stdout)?.trim().to_owned();
            let object = format!("{}:{}", digest, target.location.path);
            let blob = Command::new("git")
                .args(["-C"])
                .arg(root)
                .args(["cat-file", "blob"])
                .arg(object)
                .output()?;
            if !blob.status.success() || blob.stdout != std::fs::read(target_path)? {
                bail!("target differs from HEAD and cannot be accepted as a commit pin");
            }
            Ok(digest)
        }
        _ => bail!("unsupported algorithm/scope combination '{algorithm}/{scope:?}'"),
    }
}

fn markdown_heading_block<'a>(text: &'a str, selector: &str) -> Option<&'a [u8]> {
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
        .filter(|(_, (slug, _, _))| slug == selector)
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

pub(super) fn utc_today_string() -> String {
    let days = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs() / 86_400)
        .unwrap_or_default() as i64;
    let (year, month, day) = civil_from_days(days);
    format!("{year:04}-{month:02}-{day:02}")
}

fn civil_from_days(days: i64) -> (i32, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    (year as i32, month as u32, day as u32)
}
