use std::collections::BTreeMap;

#[derive(Debug)]
struct Heading {
    slug: String,
    level: usize,
    start: usize,
}

pub(crate) fn heading_block<'a>(text: &'a str, selector: &str) -> Option<&'a [u8]> {
    let headings = headings(text);
    let matches = headings
        .iter()
        .enumerate()
        .filter(|(_, heading)| heading.slug == selector)
        .collect::<Vec<_>>();
    let [(index, heading)] = matches.as_slice() else {
        return None;
    };
    let end = headings
        .iter()
        .skip(index + 1)
        .find(|next| next.level <= heading.level)
        .map(|next| next.start)
        .unwrap_or(text.len());
    Some(&text.as_bytes()[heading.start..end])
}

pub(crate) fn heading_slug_counts(text: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for heading in headings(text) {
        *counts.entry(heading.slug).or_default() += 1;
    }
    counts
}

fn headings(text: &str) -> Vec<Heading> {
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
                let value = trimmed[level..].trim().trim_end_matches('#').trim();
                if let Some(slug) = heading_slug(value) {
                    headings.push(Heading {
                        slug,
                        level,
                        start: offset,
                    });
                }
            }
        }
        offset += segment.len();
    }
    headings
}

pub(crate) fn heading_slug(heading: &str) -> Option<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_index_drives_both_resolution_and_ambiguity() {
        let text = "# Root\n\n## Watched Block\nvalue\n\n### Child\nchild\n\n## Next\nend\n";
        assert_eq!(heading_slug_counts(text)["watched-block"], 1);
        assert_eq!(
            std::str::from_utf8(heading_block(text, "watched-block").unwrap()).unwrap(),
            "## Watched Block\nvalue\n\n### Child\nchild\n\n"
        );

        let duplicate = format!("{text}\n## Watched Block\nother\n");
        assert_eq!(heading_slug_counts(&duplicate)["watched-block"], 2);
        assert!(heading_block(&duplicate, "watched-block").is_none());
    }

    #[test]
    fn fenced_code_headings_are_not_semantic_blocks() {
        let text = "```md\n## Hidden\n```\n\n## Visible\nbody\n";
        assert!(!heading_slug_counts(text).contains_key("hidden"));
        assert!(heading_block(text, "hidden").is_none());
        assert!(heading_block(text, "visible").is_some());
    }
}
