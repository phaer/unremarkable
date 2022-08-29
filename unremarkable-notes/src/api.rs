use anyhow::Context;
use actix_web::{get, error, App, HttpResponse, HttpServer, Responder, Result};
use actix_web::http::StatusCode;
use crate::notebooks;


#[get("/")]
async fn index() -> Result<impl Responder> {
    let notebooks = notebooks::list_notebooks()
        .context("Failed to list notebooks")
        .http_internal_error("Could not get database connection")?;
    Ok(HttpResponse::Ok().json(notebooks))
}


pub async fn start(host: String, port: u16) -> std::io::Result<()> {
    println!("Listening on http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
        .bind((host, port))?
        .run()
        .await
}


// https://www.reddit.com/r/rust/comments/ozc0m8/an_actixanyhow_compatible_error_helper_i_found/
pub trait IntoHttpError<T> {
    fn http_error(
        self,
        message: &str,
        status_code: StatusCode,
    ) -> core::result::Result<T, actix_web::Error>;

    fn http_internal_error(self, message: &str) -> core::result::Result<T, actix_web::Error>
    where
        Self: std::marker::Sized,
    {
        self.http_error(message, StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl<T, E: std::fmt::Debug> IntoHttpError<T> for core::result::Result<T, E> {
    fn http_error(
        self,
        message: &str,
        status_code: StatusCode,
    ) -> core::result::Result<T, actix_web::Error> {
        match self {
            Ok(val) => Ok(val),
            Err(err) => {
                println!("http_error: {:?}", err);
                Err(error::InternalError::new(message.to_string(), status_code).into())
            }
        }
    }
}
