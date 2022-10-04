//! # Sync
//!
//! Abstractions over the API used by the official Remarkable Connect Service as well as [rmfakecloud](https://ddvk.github.io/rmfakecloud/).

use std::io::{stdin, stdout, Write};
use std::collections::HashMap;
use snafu::{Snafu, ResultExt};
use reqwest;

use crate::utils::deserialize_empty_string_as_none;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Unable to authenticate with API: {}", source))]
    Auth {
        source: reqwest::Error,
    },
    #[snafu(display("API Error: {}", source))]
    Api {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoteItem {
    #[serde(rename="ID")]
    pub id: uuid::Uuid,
    pub version: u8,
    pub message: String,
    pub success: bool,
    #[serde(rename="BlobURLGet")]
    pub blob_url_get: String,
    #[serde(rename="BlobURLGetExpires")]
    pub blob_url_get_expires: String,
    #[serde(rename="BlobURLPut")]
    pub blob_url_put: Option<String>,
    #[serde(rename="BlobURLPutExpires")]
    pub blob_url_put_expires: Option<String>,
    pub modified_client: String,
    #[serde(rename="Type")]
    pub type_: String,
    pub vissible_name: String,  // yes, there's a typo in this api key :/
    pub current_page: i32,
    pub bookmarked: bool,
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub parent: Option<String>, // this can be a uuid OR the string "trash" (as well as an empty string)
}


#[derive(Debug)]
pub struct Client {
    pub config: Config,
    pub client: reqwest::blocking::Client
}

impl Client {
    pub fn from(mut config: Config) -> Result<Self> {
        if config.token.is_none() {
            Self::authenticate_interactively(&mut config)?
        }// else {
        //    Self::refresh_token(&mut config)?
        //}

        let client = reqwest::blocking::Client::new();
        Ok(Self {
            config,
            client,
        })
    }

    pub fn info(&self) {
        println!("{:#?}", self.config);
    }

    pub fn all(&self) -> Result<()> {
        let res = self.client.get(format!("{}/document-storage/json/2/docs", self.config.storage_host))
                             .header(reqwest::header::AUTHORIZATION,
                                     format!("Bearer {}", self.config.token.as_deref().expect("API Token not found")))
                             .send()
                             .context(ApiSnafu {})?;
        let json = res.json::<Vec<RemoteItem>>().context(ApiSnafu {})?;
        println!("{:#?}", json);
        Ok(())
    }

    fn refresh_token(config: &mut Config) -> Result<()> {
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("{}/token/json/2/user/new", config.auth_host))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", config.token.as_deref().expect("API Token not found")))
            .send()
            .context(AuthSnafu {})?;
        let res = res.error_for_status().context(AuthSnafu {})?;
        let token = Some(res.text().context(AuthSnafu {})?);
        if config.token != token {
            config.token = token;
            config.save().context(SaveTokenSnafu)?;
        }
        println!("new token: {:#?}", config.token);
        Ok(())
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
            .post(format!("{}/token/json/2/device/new", config.auth_host))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&map);
        let res = req.send().context(AuthSnafu {})?;
        let res = res.error_for_status().context(AuthSnafu {})?;
        let token = res.text() .context(AuthSnafu {})?;
        config.token = Some(token);
        Ok(())
    }
}
