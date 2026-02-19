mod chp;
mod english;
mod eu;
mod france;
mod international;
mod italia;
mod nato;
mod netherlands;
mod sweden;
mod uk;
mod japanese;

use std::io::{BufRead, BufReader};
use crate::{Code, Codes, Error, PC};
use include_dir::{include_dir, Dir};

pub(crate) use chp::Chp;
pub(crate) use english::English;
pub(crate) use eu::Eu;
pub(crate) use france::France;
pub(crate) use international::International;
pub(crate) use italia::Italia;
pub(crate) use japanese::Japanese;
pub(crate) use nato::Nato;
pub(crate) use netherlands::Netherlands;
pub(crate) use sweden::Sweden;
pub(crate) use uk::Uk;

static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/codes");

pub(crate) fn is_available_name(name: &str) -> bool {
    ASSETS.get_file(format!("{}.txt", name.to_lowercase())).is_some()
}

pub(crate) fn list_assets() -> Vec<String> {
    ASSETS.files()
        .filter_map(|entry| entry.path().to_str().map(|s| s.to_string()))
        .filter_map(|s| s.strip_suffix(".txt").map(|name| name.to_string()))
        .collect()
}

pub(crate) fn build_from_asset(name: &str) -> Result<Codes, Error> {
    let asset_name = format!("{}.txt", name.to_lowercase());
    if let Some(file) = ASSETS.get_file(asset_name) {
        build_from_reader(file.contents())
    } else {
        Err(Error::Asset(name.to_string()))
    }
}

pub(crate) fn build_from_reader(reader: impl std::io::Read) -> Result<Codes, Error> {
    let reader = BufReader::new(reader);
    let mut codes = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        let line = trim_and_strip_comments(&line);
        if line.is_empty() || line.starts_with('#') {
            continue;
        } else if let Some(base_code) = line.strip_prefix("base:") {
            let base_code = base_code.trim();
            if let Ok(base) = base_code.parse::<Codes>() {
                base.entries().for_each(|entry| codes.push(entry.clone()));
            } else {
                return Err(Error::Parse(format!("{base_code}: specified base code is not available")));
            }
        } else {
            let parts: Vec<&str> = line.splitn(2, [',', ';', ':', '=', ' ', '\t']).collect();
            if parts.len() == 2 {
                let letter = if parts[0].len() == 1 {
                    parts[0].chars().next().unwrap_or_default()
                } else if parts[0].starts_with("\\u{") && parts[0].ends_with('}') {
                    parse_unicode_letter(parts[0])
                } else {
                    continue; // Skip invalid lines
                };
                let code = parts[1].trim().to_string();
                codes.push(Code::new(letter, code));
            }
        }
    }
    Ok(Codes::new_of(PC::Null, codes))
}

fn parse_unicode_letter(s: &str) -> char {
    let hex = &s[3..s.len() - 1]; // Extract the hex part from \u{...}
    u32::from_str_radix(hex, 16)
        .ok()
        .and_then(std::char::from_u32)
        .unwrap_or_default() // Return default char if parsing fails
}

fn trim_and_strip_comments(line: &str) -> &str {
    let line = line.trim();
    let mut search_from = 0; // find the position of first '#' with no escape.
    while let Some(pos) = line[search_from..].find('#') {
        let actual_pos = search_from + pos;
        // found '#' is the first character, or is not escaped.
        if actual_pos == 0 || !line[..actual_pos].ends_with('\\') {
            return line[..actual_pos].trim();
        }
        // try next search, since the found '#' is escaped
        search_from = actual_pos + 1;
    }
    line // '#' was not found.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_assets() {
        let assets = list_assets();
        assert!(assets.contains(&"denmark.txt".to_string()));
        assert!(assets.contains(&"switzerland.txt".to_string()));
        assert_eq!(assets.len(), 6);
    }

    #[test]
    fn test_trim_and_strip_comments() {
        assert_eq!(trim_and_strip_comments("  A, Alpha  "), "A, Alpha");
        assert_eq!(trim_and_strip_comments("A, Alpha # This is a comment"), "A, Alpha");
        assert_eq!(trim_and_strip_comments("A, Alpha \\# Not a comment"), "A, Alpha \\# Not a comment");
        assert_eq!(trim_and_strip_comments("A, Alpha # Comment with escaped \\# character"), "A, Alpha");
        assert_eq!(trim_and_strip_comments("  # This is a comment line  "), ""); // Line with only a comment should return an empty string after trimming
        assert_eq!(trim_and_strip_comments("  A, Alpha # Comment with leading and trailing spaces  "), "A, Alpha"); // Comment
    }

    #[test]
    fn test_parse_unicode_letter() {
        assert_eq!(parse_unicode_letter("\\u{41}"), 'A');
        assert_eq!(parse_unicode_letter("\\u{1F600}"), '😀');
        assert_eq!(parse_unicode_letter("\\u{00C4}"), 'Ä');
        assert_eq!(parse_unicode_letter("\\u{ZZZZ}"), '\0'); // Invalid hex should return default char
    }
}