use regex::Regex;

pub fn get_year_regex() -> Regex {
    Regex::new(r"\((\d{4})\)").expect("Regex pattern should never fail")
}

pub fn get_sq_bracket_regex() -> Regex {
    Regex::new(r"\[(.*?)]").expect("Regex pattern should never fail")
}

pub fn get_ep_regex() -> Regex {
    Regex::new(r"^(.*?)(s(\d+)e(\d+(\.\d+)?))(.*?)$").expect("Regex pattern should never fail")
}

pub fn get_season_regex() -> Regex {
    Regex::new(r"season (\d+)").expect("Regex pattern should never fail")
}