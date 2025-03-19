use bigdecimal::{BigDecimal, FromPrimitive as _};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::reg::get_ep_regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Segment {
    /// [**]
    Sq(String),
    /// (**)
    Br(String),
    None(String),
    /// {**}
    Curly(String),
}

pub enum Segment2 {
    Str(String),
    SeasonEpi((u64, Episode)),
}

impl Segment2 {
    pub fn to_str(self) -> Option<String> {
        match self {
            Segment2::Str(s) => Some(s),
            Segment2::SeasonEpi(_) => None,
        }
    }
    fn is_epi(&self) -> bool {
        match self {
            Segment2::Str(_) => false,
            Segment2::SeasonEpi(_) => true,
        }
    }
    pub fn parse(items: Vec<Segment>) -> Vec<Segment2> {
        let ep_regex = get_ep_regex();
        let mut data = vec![];
        let mut end = false;
        for item in items {
            if end {
                data.push(Segment2::Str(item.as_str().to_owned()));
            }
            if let Some(captures) = ep_regex.captures(&item.as_str()) {
                let before = captures.get(1).map_or("", |m| trim(m.as_str()));
                data.push(Segment2::Str(before.to_owned()));
                let season_number: u64 = captures[3].parse().unwrap();
                let episode_number: BigDecimal = captures[4].parse().unwrap();
                let after = captures.get(6).map_or("", |m| trim(m.as_str()));
                end = true;
                let (mut episode, mut after) = Episode::from(episode_number, parse_after(&after));
                if let Episode::Single(epi) = &episode {
                    if let Some((part, af)) = part_parser(&after) {
                        after = af.unwrap_or_default().to_string();
                        episode = Episode::Part(epi.clone(), part)
                    }
                }
                data.push(Segment2::SeasonEpi((season_number, episode)));
                data.push(Segment2::Str(after));
            } else {
                data.push(Segment2::Str(item.as_str().to_owned()));
            }
        }
        data
    }
}

fn part_parser(input_string: &str) -> Option<(u64, Option<&str>)> {
    let keywords = ["cd", "dvd", "part", "pt", "disc", "disk"];
    let separators = r"[ \-_]";

    let keyword_pattern = keywords.join("|");

    let pattern = format!(
        r"^({}){}*(\d+)(?:{}+(.+))?$",
        keyword_pattern, separators, separators
    );
    let regex = Regex::new(&pattern).unwrap();

    if let Some(captures) = regex.captures(input_string) {
        if let Some(number_match) = captures.get(2) {
            if let Ok(number) = number_match.as_str().parse::<u64>() {
                let text_after = captures.get(3).map(|m| trim(m.as_str()));
                return Some((number, text_after.filter(|s| !s.is_empty())));
            }
        }
    }
    None
}

fn trim<'a>(s: &'a str) -> &'a str {
    s.trim_matches(|c: char| c == ' ' || c == '_' || c == '.' || c == '-')
}

#[derive(Debug, PartialEq, Eq)]
enum MatchType {
    DoubleDot,
    DoubleDotEquals,
    Dash,
    CommaList,
    None,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum Episode {
    List(Vec<BigDecimal>),
    Single(BigDecimal),
    Part(BigDecimal, u64),
    RangeInclusive(BigDecimal, BigDecimal),
}

impl Episode {
    fn from(first: BigDecimal, other: ParsedResult) -> (Self, String) {
        match other.match_type {
            MatchType::DoubleDot => (
                Episode::RangeInclusive(
                    first,
                    other.numbers[0].clone() + BigDecimal::from_u8(1).unwrap(),
                ),
                trim(&other.remaining).to_owned(),
            ),
            MatchType::DoubleDotEquals | MatchType::Dash => (
                Episode::RangeInclusive(first, other.numbers[0].clone()),
                trim(&other.remaining).to_owned(),
            ),
            MatchType::CommaList => {
                let mut items = vec![first];
                items.extend(other.numbers);
                (Episode::List(items), trim(&other.remaining).to_owned())
            }
            MatchType::None => (Episode::Single(first), trim(&other.remaining).to_owned()),
        }
    }
}

#[derive(Debug)]
struct ParsedResult {
    match_type: MatchType,
    numbers: Vec<BigDecimal>,
    remaining: String,
}

fn parse_after(input: &str) -> ParsedResult {
    let separators = ["\\s", "\\-", "_"];
    let separator_pattern = separators.join("|");

    let re_double_dot = Regex::new(&format!(
        r"^(\.\.=?)(-?[eE]?\d+(?:\.\d+)?)([{sep}].*)?$",
        sep = separator_pattern
    ))
    .unwrap();

    let re_dash = Regex::new(&format!(
        r"^(-[eE]?)(\d+(?:\.\d+)?)([{sep}].*)?$",
        sep = separator_pattern
    ))
    .unwrap();
    let re_comma_list = Regex::new(&format!(
        r"^,((-?[eE]?\d+(?:\.\d+)?,)+-?[eE]?\d+(?:\.\d+)?)([{sep}].*)?$",
        sep = separator_pattern
    ))
    .unwrap();

    if let Some(caps) = re_double_dot.captures(input) {
        let match_type = match &caps[1] {
            ".." => MatchType::DoubleDot,
            "..=" => MatchType::DoubleDotEquals,
            _ => MatchType::None,
        };
        let number: BigDecimal = caps[2]
            .to_lowercase()
            .strip_prefix("e")
            .unwrap_or(&caps[2])
            .parse()
            .unwrap();
        return ParsedResult {
            match_type,
            numbers: vec![number],
            remaining: caps
                .get(3)
                .map_or("".to_string(), |m| m.as_str().to_string()),
        };
    }

    if let Some(caps) = re_dash.captures(input) {
        let number: BigDecimal = caps[2]
            .to_lowercase()
            .strip_prefix("e")
            .unwrap_or(&caps[2])
            .parse()
            .unwrap();
        return ParsedResult {
            match_type: MatchType::Dash,
            numbers: vec![number],
            remaining: caps
                .get(3)
                .map_or("".to_string(), |m| m.as_str().to_string()),
        };
    }

    if let Some(caps) = re_comma_list.captures(input) {
        let numbers: Vec<BigDecimal> = caps[1]
            .split(',')
            .filter_map(|n| n.to_lowercase().strip_prefix("e").unwrap_or(n).parse().ok())
            .collect();
        return ParsedResult {
            match_type: MatchType::CommaList,
            numbers,
            remaining: caps
                .get(3)
                .map_or("".to_string(), |m| m.as_str().to_string()),
        };
    }

    ParsedResult {
        match_type: MatchType::None,
        numbers: vec![],
        remaining: input.to_string(),
    }
}

impl Segment {
    fn as_str(&self) -> &str {
        match self {
            Segment::Sq(s) => s.as_str(),
            Segment::Br(s) => s.as_str(),
            Segment::None(s) => s.as_str(),
            Segment::Curly(s) => s.as_str(),
        }
    }
    fn is_empty(&self) -> bool {
        match self {
            Segment::Sq(s) => s.is_empty(),
            Segment::Br(s) => s.is_empty(),
            Segment::None(s) => s.is_empty(),
            Segment::Curly(s) => s.is_empty(),
        }
    }
    pub fn is_none(&self) -> bool {
        match self {
            Segment::None(_) => true,
            _ => false,
        }
    }
}

pub fn parse_segments(input: &str) -> Vec<Segment> {
    let re = Regex::new(r"\[(.*?)\]|\((.*?)\)|\{(.*?)\}|([^\[\]{}()]+)").unwrap();
    let mut segments = Vec::new();

    for cap in re.captures_iter(input) {
        if let Some(m) = cap.get(1) {
            segments.push(Segment::Sq(trim(m.as_str()).to_owned())); // Square brackets
        } else if let Some(m) = cap.get(2) {
            segments.push(Segment::Br(trim(m.as_str()).to_owned())); // Parentheses
        } else if let Some(m) = cap.get(3) {
            segments.push(Segment::Curly(trim(m.as_str()).to_owned())); // Curly braces
        } else if let Some(m) = cap.get(4) {
            segments.push(Segment::None(trim(m.as_str()).to_owned())); // Plain text
        }
    }

    segments.into_iter().filter(|v| !v.is_empty()).collect()
}

pub fn segements2_to_tuple(
    items: Vec<Segment2>,
) -> Result<(String, (u64, Episode), String), String> {
    if items.iter().find(|v| v.is_epi()).is_some() {
        let mut found = None;
        let mut before = vec![];
        let mut after = vec![];
        for item in items {
            match item {
                Segment2::Str(s) => match found.is_none() {
                    true => before.push(s),
                    false => after.push(s),
                },
                Segment2::SeasonEpi(e) => {
                    found = Some(e);
                }
            }
        }
        Ok((before.join(" "), found.unwrap(), after.join(" ")))
    } else {
        Err(items
            .into_iter()
            .filter_map(|v| v.to_str())
            .collect::<Vec<_>>()
            .join(" "))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    impl PartialEq for ParsedResult {
        fn eq(&self, other: &Self) -> bool {
            self.match_type == other.match_type
                && self
                    .numbers
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    == other
                        .numbers
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<String>>()
                && self.remaining == other.remaining
        }
    }

    impl Eq for ParsedResult {}

    #[test]
    fn test_parse_segments() {
        let input = "Hello [world] this is (Rust) and {regex}!";
        let segments = parse_segments(input);
        println!("{:?}", segments);
    }

    #[test]
    fn parse_part_test() {
        assert_eq!(part_parser("part-45"), Some((45, None)));
        assert_eq!(
            part_parser("disk_100_more text"),
            Some((100, Some("more text")))
        );
        assert_eq!(part_parser(" dvd 55"), None);
        assert_eq!(part_parser("dvd 55a"), None);
        assert_eq!(part_parser("dvd 55"), Some((55, None)));

        assert_eq!(
            part_parser("cd 55 other text"),
            Some((55, Some("other text")))
        );
        assert_eq!(
            part_parser("cd 55 other text"),
            Some((55, Some("other text")))
        );
        assert_eq!(
            part_parser("cd 55 other text"),
            Some((55, Some("other text")))
        );
    }

    #[test]
    fn parse_after_test() {
        let test_cases = vec![
            (
                "..10.5-hello world",
                ParsedResult {
                    match_type: MatchType::DoubleDot,
                    numbers: vec![BigDecimal::from_str("10.5").unwrap()],
                    remaining: "-hello world".to_string(),
                },
            ),
            (
                "..=5.3 hello",
                ParsedResult {
                    match_type: MatchType::DoubleDotEquals,
                    numbers: vec![BigDecimal::from_str("5.3").unwrap()],
                    remaining: " hello".to_string(),
                },
            ),
            (
                "..=e5.3 hello",
                ParsedResult {
                    match_type: MatchType::DoubleDotEquals,
                    numbers: vec![BigDecimal::from_str("5.3").unwrap()],
                    remaining: " hello".to_string(),
                },
            ),
            (
                "-e42.7_test",
                ParsedResult {
                    match_type: MatchType::Dash,
                    numbers: vec![BigDecimal::from_str("42.7").unwrap()],
                    remaining: "_test".to_string(),
                },
            ),
            (
                "-42.7_test",
                ParsedResult {
                    match_type: MatchType::Dash,
                    numbers: vec![BigDecimal::from_str("42.7").unwrap()],
                    remaining: "_test".to_string(),
                },
            ),
            (
                ",3.1,4.2,5.3,5,8.2,9 world",
                ParsedResult {
                    match_type: MatchType::CommaList,
                    numbers: vec![
                        BigDecimal::from_str("3.1").unwrap(),
                        BigDecimal::from_str("4.2").unwrap(),
                        BigDecimal::from_str("5.3").unwrap(),
                        BigDecimal::from_str("5").unwrap(),
                        BigDecimal::from_str("8.2").unwrap(),
                        BigDecimal::from_str("9").unwrap(),
                    ],
                    remaining: " world".to_string(),
                },
            ),
            (
                ",e3.1,E4,e2,E5.3,e5,E8.2,e9 world",
                ParsedResult {
                    match_type: MatchType::CommaList,
                    numbers: vec![
                        BigDecimal::from_str("3.1").unwrap(),
                        BigDecimal::from_str("4").unwrap(),
                        BigDecimal::from_str("2").unwrap(),
                        BigDecimal::from_str("5.3").unwrap(),
                        BigDecimal::from_str("5").unwrap(),
                        BigDecimal::from_str("8.2").unwrap(),
                        BigDecimal::from_str("9").unwrap(),
                    ],
                    remaining: " world".to_string(),
                },
            ),
            (
                "Series Name A S01E01-E02 Name.mkv",
                ParsedResult {
                    match_type: MatchType::None,
                    numbers: vec![],
                    remaining: "Series Name A S01E01-E02 Name.mkv".to_string(),
                },
            ),
            (
                "-E02 Name.mkv",
                ParsedResult {
                    match_type: MatchType::Dash,
                    numbers: vec![BigDecimal::from_str("2").unwrap()],
                    remaining: " Name.mkv".to_string(),
                },
            ),
            (
                "..=E02 Name.mkv",
                ParsedResult {
                    match_type: MatchType::DoubleDotEquals,
                    numbers: vec![BigDecimal::from_str("2").unwrap()],
                    remaining: " Name.mkv".to_string(),
                },
            ),
            (
                "..E02 Name.mkv",
                ParsedResult {
                    match_type: MatchType::DoubleDot,
                    numbers: vec![BigDecimal::from_str("2").unwrap()],
                    remaining: " Name.mkv".to_string(),
                },
            ),
            (
                "-02 Name.mkv",
                ParsedResult {
                    match_type: MatchType::Dash,
                    numbers: vec![BigDecimal::from_str("2").unwrap()],
                    remaining: " Name.mkv".to_string(),
                },
            ),
            (
                "..=02 Name.mkv",
                ParsedResult {
                    match_type: MatchType::DoubleDotEquals,
                    numbers: vec![BigDecimal::from_str("2").unwrap()],
                    remaining: " Name.mkv".to_string(),
                },
            ),
            (
                "..02 Name.mkv",
                ParsedResult {
                    match_type: MatchType::DoubleDot,
                    numbers: vec![BigDecimal::from_str("2").unwrap()],
                    remaining: " Name.mkv".to_string(),
                },
            ),
            (
                ",5,6,E7 Name.mkv",
                ParsedResult {
                    match_type: MatchType::CommaList,
                    numbers: vec![
                        BigDecimal::from_str("5").unwrap(),
                        BigDecimal::from_str("6").unwrap(),
                        BigDecimal::from_str("7").unwrap(),
                    ],
                    remaining: " Name.mkv".to_string(),
                },
            ),
            (
                "random text",
                ParsedResult {
                    match_type: MatchType::None,
                    numbers: vec![],
                    remaining: "random text".to_string(),
                },
            ),
            (
                "-eng",
                ParsedResult {
                    match_type: MatchType::None,
                    numbers: vec![],
                    remaining: "-eng".to_string(),
                },
            ),
            (
                "-E02",
                ParsedResult {
                    match_type: MatchType::Dash,
                    numbers: vec![BigDecimal::from_str("2.0").unwrap()],
                    remaining: "".to_string(),
                },
            ),
        ];

        for (input, expected) in test_cases {
            let item = parse_after(input);
            assert_eq!(item.match_type, expected.match_type);
            assert_eq!(item.remaining, expected.remaining);
            assert_eq!(item.numbers, expected.numbers);
        }
    }
}
