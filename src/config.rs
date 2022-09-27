use std::{fs::File, io::Read};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use snafu::{Snafu, ResultExt};

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Unable to read file at  {}: {}", path.display(), source))]
    ReadFile {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("Unable to parse toml at {}: {}", path.display(), source))]
    ParseToml {
        source: toml::de::Error,
        path: PathBuf,
    },
}

type Result<T> = std::result::Result<T, Error>;
type Host = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Credentials {
    AuthCode { auth_code: String },
    Token{ token: String },
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: Host,
    pub description: String,
    pub id: uuid::Uuid,
    #[serde(flatten)]
    pub credentials: Credentials,
}

impl Config {
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let mut file = File::open(path).context(ReadFileSnafu { path })?;
        let mut content = String::new();
        file.read_to_string(&mut content).context(ReadFileSnafu { path })?;
        let config = toml::from_str(&content).context(ParseTomlSnafu { path })?;
        Ok(config)
    }
}
