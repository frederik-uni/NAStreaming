mod episode_guessing;
mod parser;
mod reg;
mod typ;

pub use parser::parse_library;
//TODO: refactor

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::parser::parse_library;

    #[test]
    fn test() {
        let path: PathBuf = "/Volumes/NAStreaming/Anime".into();
        //ffmpeg_next::init().expect("TODO: panic message");
        let v = parse_library(&path, true, &[], false).expect("TODO: panic message");
        println!("{}", v.len());
    }
}
