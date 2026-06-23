use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::types::*;

pub fn determine_sections(
    geom: &RectGeometry,
    all_sections: &HashMap<u32, Section>,
) -> HashSet<u32> {
    let mut in_sections: HashSet<u32> = HashSet::new();
    for (section_id, section) in all_sections {
        if section.geom.overlaps(geom) {
            in_sections.insert(*section_id);
        }
    }
    in_sections
}

/// Insert key value pari into HashMap panicing, if the key already exists
pub fn insert_unique<K, V>(map: &mut HashMap<K, V>, key: K, value: V)
where
    K: Eq,
    K: Hash,
{
    if map.contains_key(&key) {
        panic!("tried to insert cell with same id twice");
    }
    map.insert(key, value);
}
pub fn to_nice_string(input: Option<&str>) -> String {
    let Some(input) = input else {
        println!("Warning: Unnamed element");
        return "Unnamed".to_string();
    };
    // Step 1: Decode XML entities: &lt; -> <, &gt; -> >, etc.
    let html = input
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&amp;", "&")
        .replace("&apos;", "'");

    // Step 2: Strip HTML tags, converting block elements to newlines
    let mut result = String::new();
    let mut inside_tag = false;
    let mut tag_buf = String::new();

    for ch in html.chars() {
        match ch {
            '<' => {
                inside_tag = true;
                tag_buf.clear();
            }
            '>' => {
                inside_tag = false;
                // Convert block-level tags to newlines
                let tag = tag_buf.trim().to_lowercase();
                if tag == "br"
                    || tag == "/br"
                    || tag.starts_with("br ")
                    || tag == "/div"
                    || tag == "/p"
                {
                    result.push('\n');
                }
                tag_buf.clear();
            }
            _ if inside_tag => tag_buf.push(ch),
            _ => result.push(ch),
        }
    }

    // Step 3: Decode HTML entities in the text content
    let result = result
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"");

    // Step 4: Clean up extra newlines
    let result = result
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<_>>()
        .join(" ");

    // Remove leading/trailing newlines
    result.split_whitespace().collect::<Vec<&str>>().join(" ")
    // result.trim_matches('\n').to_string()
}
