//! A library for phonetic codes, such as the NATO phonetic alphabet.
//! This library provides a way to get the phonetic code for a given character, as well as to get the phonetic codes for a given word.
//! The library includes several predefined phonetic alphabets, such as the NATO alphabet, the International alphabet, and the Italian alphabet. It also allows for custom phonetic alphabets to be created by providing a base alphabet and a list of substitutions.
//! 
//! ### Examples
//! 
//! #### Get the phonetic code for a character
//! 
//! The `code` function returns the phonetic code for a given character.
//! If the character is not found in the phonetic alphabet, it returns `None`.
//! 
//! ```rust
//! assert_eq!(phonetic_code::code('a').map(|c| c.code()), Some("Alpha".to_string()));
//! assert_eq!(phonetic_code::code('b').map(|c| c.code()), Some("Bravo".to_string()));
//! assert_eq!(phonetic_code::code('c').map(|c| c.code()), Some("Charlie".to_string()));
//! ```
//! 
//! #### Get the phonetic codes for a word
//! 
//! ```rust
//! use phonetic_code::{CodesBuilder, PhoneticCode};
//! 
//! let codes = CodesBuilder::build(PhoneticCode::Uk);
//! assert_eq!(codes.code('a').map(|c| c.code()), Some("Able".to_string()));
//! assert_eq!(codes.code('b').map(|c| c.code()), Some("Baker".to_string()));
//! ```
use std::collections::HashMap;
use std::fmt::Display;
use std::{fs::File, path::Path, str::FromStr, sync::OnceLock};
use std::io::{BufRead, BufReader};

use clap::ValueEnum;

mod codes;

static NATO: OnceLock<Codes> = OnceLock::new();

/// Returns the phonetic code for a given character.
/// If the character is not found in the phonetic alphabet, it returns `None`.
pub fn code(letter: char) -> Option<&'static Code> {
    NATO.get_or_init(|| {
        PhoneticCode::of(PhoneticCode::Nato)
    }).code(letter)
}

/// Returns an iterator over the characters in the given string and their corresponding phonetic codes.
/// The iterator yields a tuple of the character and an `Option<&Code>`, where the
/// `Option<&Code>` is `Some(&Code)` if the character has a corresponding phonetic code, and `None` otherwise.
pub fn convert(words: &str) -> impl Iterator<Item = (char, Option<&Code>)> {
    NATO.get_or_init(|| {
        PhoneticCode::of(PhoneticCode::Nato)
    }).convert(words)
}

/// Returns an iterator over all the phonetic codes in the NATO alphabet.
pub fn entries() -> impl Iterator<Item = &'static Code> {
    NATO.get_or_init(|| {
        PhoneticCode::of(PhoneticCode::Nato)
    }).entries()
}

/// Predefined phonetic codes.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ValueEnum)]
pub enum PhoneticCode {
    /// 
    Chp,
    English,
    Eu,
    France,
    Indonesia,
    International,
    Italia,
    /// [NATO Phonetic Alphabet, Code, & Signals](https://www.nato.int/content/dam/nato/webready/news/2010-2019/2017/12/21/20180111_nato-alphabet-sign-signal.pdf).
    Nato,
    Netherlands,
    Philippines,
    Sweden,
    Uk,
    USAAirpots,
    Japanese,
}

impl Display for PhoneticCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhoneticCode::Chp => write!(f, "chp"),
            PhoneticCode::English => write!(f, "english"),
            PhoneticCode::Eu => write!(f, "eu"),
            PhoneticCode::France => write!(f, "france"),
            PhoneticCode::International => write!(f, "international"),
            PhoneticCode::Italia => write!(f, "italia"),
            PhoneticCode::Nato => write!(f, "nato"),
            PhoneticCode::Netherlands => write!(f, "netherlands"),
            PhoneticCode::Philippines => write!(f, "philippines"),
            PhoneticCode::Sweden => write!(f, "sweden"),
            PhoneticCode::Uk => write!(f, "uk"),
            PhoneticCode::USAAirpots => write!(f, "usaairpots"),
            PhoneticCode::Japanese => write!(f, "japanese"),
            PhoneticCode::Indonesia => write!(f, "indonesia"),
        }
    }
}

impl FromStr for Codes {
    type Err = String;
    /// Converts a string into a [`Codes`] struct, by matching the string against the predefined phonetic code names (see [`PhoneticCode`]).
    /// 
    /// If the string matches a known phonetic code name, it returns a `Codes` struct, and does not match, it returns an error message.
    /// The matching is case-insensitive, so for example "nato", "NATO", and "NaTo" would all match the [`PhoneticCode::Nato`] variant.
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "chp" => Ok(PhoneticCode::of(PhoneticCode::Chp)),
            "english" => Ok(PhoneticCode::of(PhoneticCode::English)),
            "eu" => Ok(PhoneticCode::of(PhoneticCode::Eu)),
            "france" => Ok(PhoneticCode::of(PhoneticCode::France)),
            "international" => Ok(PhoneticCode::of(PhoneticCode::International)),
            "italia" => Ok(PhoneticCode::of(PhoneticCode::Italia)),
            "nato" => Ok(PhoneticCode::of(PhoneticCode::Nato)),
            "netherlands" => Ok(PhoneticCode::of(PhoneticCode::Netherlands)),
            "philippines" => Ok(PhoneticCode::of(PhoneticCode::Philippines)),
            "sweden" => Ok(PhoneticCode::of(PhoneticCode::Sweden)),
            "uk" => Ok(PhoneticCode::of(PhoneticCode::Uk)),
            "usaairpots" => Ok(PhoneticCode::of(PhoneticCode::USAAirpots)),
            "japanese" => Ok(PhoneticCode::of(PhoneticCode::Japanese)),
            "indonesia" => Ok(PhoneticCode::of(PhoneticCode::Indonesia)),
            _ => Err(format!("{s}: Unknown phonetic code name")),
        }
    }
}

impl PhoneticCode {
    fn of(self) -> Codes{
        match self {
            PhoneticCode::Chp => Codes::new_of(PC::Chp(codes::Chp::new()), Vec::new()),
            PhoneticCode::English => Codes::new_of(PC::English(codes::English::new()), Vec::new()),
            PhoneticCode::Eu => Codes::new_of(PC::Eu(codes::Eu::new()), Vec::new()),
            PhoneticCode::France => Codes::new_of(PC::France(codes::France::new()), Vec::new()),
            PhoneticCode::International => Codes::new_of(PC::International(codes::International::new()), Vec::new()),
            PhoneticCode::Italia => Codes::new_of(PC::Italia(codes::Italia::new()), Vec::new()),
            PhoneticCode::Nato => Codes::new_of(PC::Nato(codes::Nato::new()), Vec::new()),
            PhoneticCode::Netherlands => Codes::new_of(PC::Netherlands(codes::Netherlands::new()), Vec::new()),
            PhoneticCode::Sweden => Codes::new_of(PC::Sweden(codes::Sweden::new()), Vec::new()),
            PhoneticCode::Uk => Codes::new_of(PC::Uk(codes::Uk::new()), Vec::new()),
            PhoneticCode::Japanese => Codes::new_of(PC::Japanese(codes::Japanese::new()), Vec::new()),
            // The following are custom phonetic codes that are based on the NATO alphabet, but with some substitutions for specific characters.
            PhoneticCode::USAAirpots => Codes::new_of(PC::Nato(codes::Nato::new()), vec![Code::new('D', "Dixie")]),
            PhoneticCode::Indonesia => Codes::new_of(PC::Nato(codes::Nato::new()), vec![Code::new('L', "London")]),
            PhoneticCode::Philippines => Codes::new_of(PC::Nato(codes::Nato::new()), vec![Code::new('H', "Hawk")]),
        }
    }
}

/// Represents a character and its corresponding phonetic code.
/// The `Code` struct contains two fields: `alphabet`, which is the character being represented,
/// and `code`, which is the phonetic code corresponding to that character.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Code {
    /// The character being represented.
    letter: char,
    /// The phonetic code corresponding to the character.
    code: String,
}

impl Code {
    pub(crate) fn new<S: AsRef<str>>(letter: char, code: S) -> Self {
        Code { letter, code: code.as_ref().to_string() }
    }

    pub fn letter(&self) -> char {
        self.letter
    }

    pub fn code(&self) -> String {
        self.code.to_string()
    }
}

pub struct CodesBuilder {
}

impl CodesBuilder {
    /// Creates a new `Codes` struct with the given rule name.
    pub fn build(code: PhoneticCode) -> Codes {
        PhoneticCode::of(code)
    }

    /// Creates a new `Codes` struct with the given base alphabet and substitutions.
    pub fn build_with(base: PhoneticCode, codes: Vec<Code>) -> Codes {
        let (pc, codes) = match base {
            PhoneticCode::Chp => (PC::Chp(codes::Chp::new()), codes),
            PhoneticCode::English => (PC::English(codes::English::new()), codes),
            PhoneticCode::Eu => (PC::Eu(codes::Eu::new()), codes),
            PhoneticCode::France => (PC::France(codes::France::new()), codes),
            PhoneticCode::International => (PC::International(codes::International::new()), codes),
            PhoneticCode::Italia => (PC::Italia(codes::Italia::new()), codes),
            PhoneticCode::Nato => (PC::Nato(codes::Nato::new()), codes),
            PhoneticCode::Netherlands => (PC::Netherlands(codes::Netherlands::new()), codes),
            PhoneticCode::Sweden => (PC::Sweden(codes::Sweden::new()), codes),
            PhoneticCode::Uk => (PC::Uk(codes::Uk::new()), codes),
            PhoneticCode::Japanese => (PC::Japanese(codes::Japanese::new()), codes),
            // The following are custom phonetic codes that are based on the NATO alphabet, but with some substitutions for specific characters.
            PhoneticCode::USAAirpots => (PC::Nato(codes::Nato::new()), codes.into_iter().chain(vec![Code::new('D', "Dixie")]).collect()),
            PhoneticCode::Indonesia => (PC::Nato(codes::Nato::new()), codes.into_iter().chain(vec![Code::new('L', "London")]).collect()),
            PhoneticCode::Philippines => (PC::Nato(codes::Nato::new()), codes.into_iter().chain(vec![Code::new('H', "Hawk")]).collect()),            
        };
        Codes::new_of(pc, codes)
    }

    /// Creates a new `Codes` struct by reading phonetic codes from a file.
    /// The file should contain lines in the format "A, Alfa", where the first part is the character and the second part is the phonetic code.
    /// 
    /// The delimiter between the character and the code accepts a comma, semicolon, colon, equals sign, or tab.
    /// Lines that are empty or start with a '#' character are ignored as comments.
    /// 
    /// Note that, the each line must be splitted into two parts by the delimiter, otherwise it will be ignored.
    pub fn build_from_file<P: AsRef<Path>>(path: P) -> Result<Codes, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut codes = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let line = trim_and_strip_comments(&line);
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split([',', ';', ':', '=', '\t']).collect();
            if parts.len() == 2 {
                let alphabet = parts[0].chars().next().unwrap_or_default();
                let code = parts[1].trim().to_string();
                codes.push(Code::new(alphabet, code));
            }
        }
        Ok(Codes::new_of(PC::Null, codes))
    }
}

/// Represents a collection of phonetic codes.
/// 
/// To construct a `Codes` instance, use [`CodesBuilder`] struct,
/// which provides methods for creating a `Codes` struct from a predefined phonetic code ([`PheneticCode`]).
/// Otherwise, `parse` method of [`Codes`] from a string representation of a predefined phonetic code name.
/// 
/// ### Examples
/// 
/// #### Create a `Codes` struct from a predefined phonetic code
/// 
/// ```rust
/// use phonetic_code::{CodesBuilder, PhoneticCode};
/// 
/// let nato = CodesBuilder::build(PhoneticCode::Nato);
/// let uk = CodesBuilder::build(PhoneticCode::Uk);
/// ```
/// 
/// #### Create a `Codes` struct from a file
/// 
/// ```rust
/// use phonetic_code::{CodesBuilder, PhoneticCode};
/// 
/// let codes = CodesBuilder::build_from_file("testdata/custom_codes.txt")
///     .expect("Failed to read phonetic codes from file");
/// ```
/// 
/// #### Create a `Codes` struct from a `parse` method
/// 
/// ```rust
/// use phonetic_code::Codes;
/// 
/// let eu = "eu".parse::<Codes>()
///     .expect("Failed to read phonetic codes from file");
/// ```
pub struct Codes {
    codes: Vec<Code>,
}

impl Codes {
    fn new_of(base: PC, codes: Vec<Code>) -> Codes {
        let mut map = HashMap::new();
        for code in base.into_entries() {
            map.insert(code.letter(), code);
        }
        for code in codes {
            map.insert(code.letter(), code);
        }
        let mut codes = map.into_values().collect::<Vec<_>>();
        codes.sort_by(|a, b| a.letter().cmp(&b.letter()));
        Codes { codes }
    }

    /// Returns the phonetic code for a given character.
    pub fn code(&self, c: char) -> Option<&Code> {
        let cc = c.to_ascii_uppercase();
        self.codes.iter().find(|code| code.letter == cc)
    }

    /// Returns an iterator over all the phonetic codes in the `Codes` struct, including both the base alphabet and any substitutions.
    pub fn entries(&self) -> impl Iterator<Item = &Code> {
        self.codes.iter()
    }

    /// Converts a string into an iterator of characters and their corresponding phonetic codes.
    pub fn convert(&self, words: &str) -> impl Iterator<Item = (char, Option<&Code>)> {
        words.chars().map(move |c| (c, self.code(c)))
    }
}

enum PC {
    Chp(codes::Chp),
    English(codes::English),
    Eu(codes::Eu),
    France(codes::France),
    International(codes::International),
    Italia(codes::Italia),
    Japanese(codes::Japanese),
    Nato(codes::Nato),
    Netherlands(codes::Netherlands),
    Sweden(codes::Sweden),
    Uk(codes::Uk),
    Null,
}

impl PC {
    fn into_entries(self) -> Box<dyn Iterator<Item = Code>> {
        match self {
            PC::Chp(alphabet) => Box::new(alphabet.into_entries()),
            PC::English(alphabet) => Box::new(alphabet.into_entries()),
            PC::Eu(alphabet) => Box::new(alphabet.into_entries()),
            PC::France(alphabet) => Box::new(alphabet.into_entries()),
            PC::International(alphabet) => Box::new(alphabet.into_entries()),
            PC::Italia(alphabet) => Box::new(alphabet.into_entries()),
            PC::Japanese(alphabet) => Box::new(alphabet.into_entries()),
            PC::Nato(alphabet) => Box::new(alphabet.into_entries()),
            PC::Netherlands(alphabet) => Box::new(alphabet.into_entries()),
            PC::Sweden(alphabet) => Box::new(alphabet.into_entries()),
            PC::Uk(alphabet) => Box::new(alphabet.into_entries()),
            PC::Null => Box::new([].into_iter()),
        }
    }
}

trait PhoneticAlphabet {
    fn into_entries(self) -> impl Iterator<Item = Code>;
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
    fn test_code() {
        assert_eq!(crate::code('a').map(|c| c.code()), Some("Alpha".to_string()));
        assert_eq!(crate::code('b').map(|c| c.code()), Some("Bravo".to_string()));
        assert_eq!(crate::code('c').map(|c| c.code()), Some("Charlie".to_string()));
    }

    #[test]
    fn test_nato() {
        let usa_airpots = CodesBuilder::build(PhoneticCode::Nato);
        assert_eq!(crate::code('a').map(|c| c.code()), Some("Alpha".to_string()));
        assert_eq!(crate::code('b').map(|c| c.code()), Some("Bravo".to_string()));
        assert_eq!(crate::code('c').map(|c| c.code()), Some("Charlie".to_string()));
        assert_eq!(usa_airpots.entries().count(), 29);
    }

    #[test]
    fn test_predefined() {
        let usa_airpots = CodesBuilder::build(PhoneticCode::USAAirpots);
        assert_eq!(usa_airpots.code('D').map(|c| c.code()), Some("Dixie".to_string()));
        assert_eq!(usa_airpots.code('A').map(|c| c.code()), Some("Alpha".to_string()));
        assert_eq!(usa_airpots.entries().count(), 29);
    }

    #[test]
    fn test_from_file() {
        let codes = CodesBuilder::build_from_file("testdata/custom_codes.txt")
            .expect("Failed to read phonetic codes from file");
        assert_eq!(codes.code('A').map(|c| c.code()), Some("Arctic".to_string()));
        assert_eq!(codes.code('B').map(|c| c.code()), Some("Bishop".to_string()));
        assert_eq!(codes.entries().count(), 26);
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
}
