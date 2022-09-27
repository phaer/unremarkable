//! # Sync
//!
//! Abstractions over the API used by the official Remarkable Connect Service as well as [rmfakecloud](https://ddvk.github.io/rmfakecloud/).

use std::collections::HashMap;
use snafu::{Snafu, ResultExt};
use reqwest;

use crate::config::{Config, Credentials};

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Unable to authenticate with API: {}", source))]
    Auth {
        source: reqwest::Error,
    },

}

pub type Result<T> = std::result::Result<T, Error>;


#[derive(Debug)]
pub struct Client {
    pub config: Config,
    pub client: reqwest::blocking::Client
}

type Token = String;

impl Client {
    pub fn from(config: Config) -> Result<Self> {
        let token = match &config.credentials {
            Credentials::AuthCode { auth_code } =>
                Self::auth_by_code(
                    &config,
                    auth_code
                )?,
            Credentials::Token { token } => token.to_string(),
        };
        println!("token: {}", token);

        let client = reqwest::blocking::Client::new();
        Ok(Self {
            config,
            client,
        })
    }

    pub fn info(&self) {
        println!("{:#?}", self.config);
    }

    fn auth_by_code(config: &Config, auth_code: &str) -> Result<Token> {
        let client = reqwest::blocking::Client::new();
        let mut map = HashMap::new();
        let id = config.id.to_string();
        println!("id: {:#?}", id);
        map.insert("code", auth_code);
        map.insert("deviceDesc", &config.description);
        map.insert("deviceID", &id);

        let req = client
            .post(format!("{}/token/json/2/device/new", config.host))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&map);
        println!("req: {:#?}", req);
        let res = req.send()
            .context(AuthSnafu {})?;
        let res = res.error_for_status()
            .context(AuthSnafu {})?;
        let token = res.text()
            .context(AuthSnafu {})?;
        println!("Foo {}", token, );
        Ok(token)
    }
}
