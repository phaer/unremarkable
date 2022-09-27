//! # Sync
//!
//! Abstractions over the API used by the official Remarkable Connect Service as well as [rmfakecloud](https://ddvk.github.io/rmfakecloud/).

use std::io::{stdin, stdout, Write};
use std::collections::HashMap;
use snafu::{Snafu, ResultExt};
use reqwest;

use crate::config::Config;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Unable to authenticate with API: {}", source))]
    Auth {
        source: reqwest::Error,
    },
    #[snafu(display("Could not read API token: {}", source))]
    ReadToken {
        source: std::io::Error,
    },
    #[snafu(display("Could not save Config with API token: {}", source))]
    SaveToken {
        source: crate::config::Error,
    },

}

pub type Result<T> = std::result::Result<T, Error>;


#[derive(Debug)]
pub struct Client {
    pub config: Config,
    pub client: reqwest::blocking::Client
}

impl Client {
    pub fn from(mut config: Config) -> Result<Self> {
        if config.token.is_none() {
            Self::authenticate_interactively(&mut config)?
        };

        let client = reqwest::blocking::Client::new();
        Ok(Self {
            config,
            client,
        })
    }

    pub fn info(&self) {
        println!("{:#?}", self.config);
    }


    fn authenticate_interactively(config: &mut Config) -> Result<()> {
        let reader = stdin();
        let mut auth_code: String = String::new();
        print!("Auth Code: ");
        stdout().flush().context(ReadTokenSnafu)?;
        reader.read_line(&mut auth_code).context(ReadTokenSnafu)?;

        println!("AUTH_CODE {:#?}", auth_code);
        Self::authenticate_by_code(config, auth_code.trim_end())?;
        config.save().context(SaveTokenSnafu)?;
        Ok(())
    }

    fn authenticate_by_code(config: &mut Config, auth_code: &str) -> Result<()> {
        let client = reqwest::blocking::Client::new();
        let mut map = HashMap::new();
        let id = config.id.to_string();
        map.insert("code", auth_code);
        map.insert("deviceDesc", &config.description);
        map.insert("deviceID", &id);

        let req = client
            .post(format!("{}/token/json/2/device/new", config.host))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&map);
        let res = req.send()
            .context(AuthSnafu {})?;
        let res = res.error_for_status()
            .context(AuthSnafu {})?;
        let token = res.text()
            .context(AuthSnafu {})?;
        config.token = Some(token);
        Ok(())
    }
}
