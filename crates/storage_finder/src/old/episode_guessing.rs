use regex::Regex;

pub fn episode_guess(s: String) -> (Option<i32>, Option<i32>) {
    let func = vec![extract_season_episode_dash, extract_season_string_episode_dash];
    for func in func {
        if let Some((s, e)) = func(&s.to_lowercase()) {
            return (Some(s), Some(e));
        }
    }
    let func_epi_only = vec![exract_ep, extract_episode_string, extract_episode_dash_underscore, extract_episode_dash];
    for func in func_epi_only {
        if let Some(e) = func(&s.to_lowercase()) {
            return (None, Some(e));
        }
    }
    (None, None)
}

fn extract_episode_dash(filename: &str) -> Option<i32> {
    extract_default(filename, r" - (\d+)")
}

fn extract_episode_dash_underscore(filename: &str) -> Option<i32> {
    extract_default(filename, r"_-_(\d+)")
}

fn extract_episode_string(filename: &str) -> Option<i32> {
    extract_default(&filename.to_lowercase(), r"episode (\d+)")
}

fn extract_season_episode_dash(filename: &str) -> Option<(i32, i32)> {
    extract_double(filename, r"s(\d+) - (\d+)")
}

fn extract_season_string_episode_dash(filename: &str) -> Option<(i32, i32)> {
    extract_double(filename, r"season (\d+) - (\d+)")
}

fn exract_ep(filename: &str) -> Option<i32> {
    extract_default(filename.to_lowercase().as_str(), r"ep(\d+)")
}

fn extract_double(filename: &str, re: &str) -> Option<(i32, i32)> {
    let re = Regex::new(re).expect("Regex");
    if let Some(captures) = re.captures(filename) {
        let season = captures[1].parse().unwrap();
        let episode = captures[2].parse().unwrap();
        Some((season, episode))
    } else {
        None
    }
}

fn extract_default(filename: &str, re: &str) -> Option<i32> {
    let re = Regex::new(re).expect("Regex");
    if let Some(captures) = re.captures(filename) {
        captures[1].parse().ok()
    } else {
        None
    }
}