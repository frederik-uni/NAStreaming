mod episode_guessing;
mod parser;
mod path_tree;
mod reg;
mod resolution;
mod segments;
mod suffixes;

pub use bigdecimal::BigDecimal;
pub use bigdecimal::ParseBigDecimalError;
pub use parser::parse_library;
pub use parser::Entry;
pub use path_tree::PathTree;
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

    #[tokio::test]
    async fn tesparse_ss() {
        let path: PathBuf = "/Users/frederik/movie_files".into();

        let v = parse_library(&path, &Default::default()).await;
        println!("{}", v.len());
    }
}
