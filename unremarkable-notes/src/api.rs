use anyhow::Context;
use actix_web::{web, get, error, App, HttpResponse, HttpServer, Responder, Result};
use actix_web::http::StatusCode;
use actix_files::NamedFile;
use crate::notebooks;


#[get("/")]
async fn notebook_list() -> Result<impl Responder> {
    let notebooks = notebooks::list_notebooks()
        .context("Failed to list notebooks")
        .http_internal_error("Could not list notebooks")?;
    Ok(HttpResponse::Ok().json(notebooks))
}

#[get("/show/{id}")]
async fn notebook_detail(id: web::Path<String>) -> Result<impl Responder> {
    let notebook = notebooks::get_notebook_by_id(id.to_string())
        .http_internal_error("Could not get notebook")?;
    Ok(HttpResponse::Ok().json(notebook))
}

#[get("/show/{id}.pdf")]
async fn notebook_pdf(id: web::Path<String>) -> Result<impl Responder> {
    let notebook = notebooks::get_notebook_by_id(id.to_string())
        .http_internal_error("Could not get notebook")?;
    let output_file = "/home/phaer/src/remarkable/test.pdf";
    notebook
        .to_pdf(output_file)
        .http_internal_error("Could not render notebook")?;
    Ok(NamedFile::open_async(output_file).await?
       .disable_content_disposition())
}

pub async fn start(host: String, port: u16) -> std::io::Result<()> {
    println!("Listening on http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .service(notebook_list)
            .service(notebook_pdf)
            .service(notebook_detail)
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
