mod episode_guessing;
mod parser;
mod reg;
mod resolution;
mod segments;
mod suffixes;

pub use parser::parse_library;
pub use resolution::Resolutions;
pub use segments::Episode;
pub use suffixes::Cut;
pub use suffixes::FileType;
pub use suffixes::Kind;
pub use suffixes::ThreeD;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::parser::parse_library;

    #[test]
    fn test() {
        let path: PathBuf = "/Volumes/NAStreaming/Anime".into();
        let v = parse_library(&path, &Default::default());
        println!("{}", v.len());
    }
}
