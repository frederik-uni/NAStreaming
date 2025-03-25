use std::fmt::Display;

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash)]
pub struct Resolutions {
    pub width: Option<u32>,
    pub height: u32,
}

impl Display for Resolutions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.width {
            Some(width) => write!(f, "{} × {}", width, self.height),
            None => write!(f, "{}p", self.height),
        }
    }
}

impl Resolutions {
    pub fn from_str(input: &str) -> Option<(Self, String)> {
        let pattern_str = format!(
            r"(?i)(?P<width>\d+)\s*({})\s*(?P<height>{})",
            Self::join().join("|"),
            Self::heights()
                .iter()
                .map(|&h| h.to_string())
                .collect::<Vec<String>>()
                .join("|")
        );
        let regex = Regex::new(&pattern_str).unwrap();

        if let Some(captures) = regex.captures(input) {
            let width = captures["width"].parse::<u32>().unwrap();
            let height = captures["height"].parse::<u32>().unwrap();
            let leftover_str = regex.replace(input, "").to_string();
            return Some((
                Self {
                    width: Some(width),
                    height,
                },
                leftover_str,
            ));
        }

        let resolutions = Self::heights();
        for resolution in resolutions {
            if input.contains(&format!("{}p", resolution)) {
                let modified_input = input.replace(&format!("{}p", resolution), "");

                return Some((
                    Self {
                        width: None,
                        height: resolution,
                    },
                    modified_input,
                ));
            }
        }

        None
    }
    fn heights() -> Vec<u32> {
        vec![240, 360, 480, 540, 720, 1080, 1440, 2160, 4320]
    }

    fn join() -> Vec<&'static str> {
        vec!["x", " x ", "×", " × "]
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod tests {
        use crate::resolution::Resolutions;

        #[test]
        fn test_standard_resolution_with_x() {
            let input = "1920x1080";
            let expected = Some((
                Resolutions {
                    width: Some(1920),
                    height: 1080,
                },
                "".to_string(),
            ));
            assert_eq!(Resolutions::from_str(input), expected);
        }

        #[test]
        fn test_standard_resolution_with_spaces() {
            let input = "1920 x 1080";
            let expected = Some((
                Resolutions {
                    width: Some(1920),
                    height: 1080,
                },
                "".to_string(),
            ));
            assert_eq!(Resolutions::from_str(input), expected);
        }

        #[test]
        fn test_resolution_with_multiplication_sign() {
            let input = "1920×1080";
            let expected = Some((
                Resolutions {
                    width: Some(1920),
                    height: 1080,
                },
                "".to_string(),
            ));
            assert_eq!(Resolutions::from_str(input), expected);
        }

        #[test]
        fn test_resolution_with_multiplication_sign_and_spaces() {
            let input = "1920 × 1080";
            let expected = Some((
                Resolutions {
                    width: Some(1920),
                    height: 1080,
                },
                "".to_string(),
            ));
            assert_eq!(Resolutions::from_str(input), expected);
        }

        #[test]
        fn test_resolution_with_p_suffix() {
            let input = "1080p";
            let expected = Some((
                Resolutions {
                    width: None,
                    height: 1080,
                },
                "".to_string(),
            ));
            assert_eq!(Resolutions::from_str(input), expected);
        }

        #[test]
        fn test_resolution_with_extra_text() {
            let input = "Video in 1920x1080 quality";
            let expected = Some((
                Resolutions {
                    width: Some(1920),
                    height: 1080,
                },
                "Video in  quality".to_string(),
            ));
            assert_eq!(Resolutions::from_str(input), expected);
        }

        #[test]
        fn test_resolution_with_only_height() {
            let input = "720p clip";
            let expected = Some((
                Resolutions {
                    width: None,
                    height: 720,
                },
                " clip".to_string(),
            ));
            assert_eq!(Resolutions::from_str(input), expected);
        }

        #[test]
        fn test_invalid_resolution() {
            let input = "Some random text without resolution";
            assert_eq!(Resolutions::from_str(input), None);
        }

        #[test]
        fn test_partial_match() {
            let input = "My video is 1280x";
            assert_eq!(Resolutions::from_str(input), None);
        }
    }
}
