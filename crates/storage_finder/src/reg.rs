use regex::Regex;

pub fn get_year_regex() -> Regex {
    Regex::new(r"\((\d{4})\)").expect("Regex pattern should never fail")
}

pub fn get_sq_bracket_regex() -> Regex {
    Regex::new(r"\[(.*?)]").expect("Regex pattern should never fail")
}

pub fn get_ep_regex() -> Regex {
    Regex::new(r"(?i)^(.*?)(s(\d+)e(\d+(\.\d+)?))(.*?)$").expect("Regex pattern should never fail")
}

pub fn get_season_regex() -> Regex {
    Regex::new(r"season (\d+)").expect("Regex pattern should never fail")
}

pub struct RegexRegistry {
    pub season_regex: Regex,
    pub ep_regex: Regex,
    pub sq_bracket_regex: Regex,
    pub year_regex: Regex,
}

impl Default for RegexRegistry {
    fn default() -> Self {
        Self {
            season_regex: get_season_regex(),
            ep_regex: get_ep_regex(),
            sq_bracket_regex: get_sq_bracket_regex(),
            year_regex: get_year_regex(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::get_ep_regex;

    #[test]
    fn test_season_episode_regex() {
        let re = get_ep_regex();

        let cases = vec![
            ("s02e10", true),
            ("My Show S01E02 Ending", true),
            ("random text s5e3 more text", true),
            ("s12e4.5", true),
            ("season 02 episode 10", false),
            ("s2e", false),
            ("e02s01", false),
        ];

        for (input, expected) in cases {
            let is_match = re.is_match(input);
            assert_eq!(is_match, expected, "Failed on input: {}", input);
        }
    }
}
