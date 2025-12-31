use std::str::FromStr;

use serde::{Deserialize, Serialize};

fn slice_to_string_vec(s: &[&str]) -> Vec<String> {
    s.iter().map(|el| el.to_string()).collect()
}
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub project: Project,
    pub display: Display,
    pub exclusions: Exclusions,
}

impl Config {
    pub fn default() -> Config {
        Config {
            project: {
                Project {
                    name: "DEFAULT".to_string(),
                    prefix: "DEFAULT".to_string(),
                    prefix_style: PrefixStyle::Prefix,
                    numbering: Numbering { start: 0, step: 1 },
                    status: slice_to_string_vec(&["TODO", "IN_PROGRESS", "TESTING", "DONE"]),
                    generation_keywords: slice_to_string_vec(&["TODO", "FIXME"]),
                    comment_symbols: slice_to_string_vec(&["//", "///", "#", ";;"]),
                }
            },
            display: {
                Display {
                    context: { Context { pre: 5, post: 5 } },
                    empty_lines: false,
                }
            },
            exclusions: {
                Exclusions {
                    list: slice_to_string_vec(&[".git/*", ".svn/*"]),
                }
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub prefix_style: PrefixStyle,
    pub prefix: String,
    pub numbering: Numbering,
    pub status: Vec<String>,
    pub generation_keywords: Vec<String>,
    pub comment_symbols: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum PrefixStyle {
    PrefixNum,
    Prefix,
    Num,
    Off,
}

pub struct PrefixStyleErr;

impl FromStr for PrefixStyle {
    type Err = PrefixStyleErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err(PrefixStyleErr);
        }
        match s.trim() {
            "prefix_num" => Ok(PrefixStyle::PrefixNum),
            "prefix" => Ok(PrefixStyle::Prefix),
            "num" => Ok(PrefixStyle::Num),
            "off" => Ok(PrefixStyle::Off),
            _ => Err(PrefixStyleErr),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Numbering {
    pub start: u8,
    pub step: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Display {
    pub context: Context,
    pub empty_lines: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Context {
    pub pre: u8,
    pub post: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Exclusions {
    pub list: Vec<String>,
}
