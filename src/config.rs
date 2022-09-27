use std::io::Write;
use std::{fs::File, fs::OpenOptions, io::Read};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use snafu::{Snafu, ResultExt};

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Unable to open file at  {}: {}", path.display(), source))]
    OpenFile {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("Unable to parse toml at {}: {}", path.display(), source))]
    ParseToml {
        source: toml::de::Error,
        path: PathBuf,
    },
    #[snafu(display("Unable to serialize toml at {}: {}", path.display(), source))]
    SerializeToml {
        source: toml::ser::Error,
        path: PathBuf,
    },

}

type Result<T> = std::result::Result<T, Error>;
pub type Host = String;
pub type Token = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: Host,
    pub description: String,
    pub id: uuid::Uuid,
    pub token: Option<Token>,
    #[serde(skip)]
    pub path: PathBuf,
}

impl Config {
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let mut file = File::open(path).context(OpenFileSnafu { path })?;
        let mut content = String::new();
        file.read_to_string(&mut content).context(OpenFileSnafu { path })?;
        let mut config: Config = toml::from_str(&content).context(ParseTomlSnafu { path })?;
        config.path = path.clone();
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = &self.path;
        let mut file = OpenOptions::new()
            .write(true)
            .open(path)
            .context(OpenFileSnafu { path })?;
        let config = toml::to_vec(&self).context(SerializeTomlSnafu { path })?;
        file.write_all(&config).context(OpenFileSnafu { path })?;
        Ok(())
    }
}
