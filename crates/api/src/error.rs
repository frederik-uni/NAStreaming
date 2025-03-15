pub type StartUpResult<T> = Result<T, StartUpError>;

#[derive(Debug)]
pub enum StartUpError {
    DisplayConfig(toml::ser::Error),
    ParseConfig(toml::de::Error),
    CreateConfig(std::io::Error),
    ReadConfig(std::io::Error),
}
