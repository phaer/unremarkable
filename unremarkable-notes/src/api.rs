use std::{fs::File, io::Read};
use std::io::BufReader;
use anyhow::Context;
use crate::notebooks::{self, Notebook};

use poem::{
    listener::TcpListener,
    Result,
    Route,
    Server,
    web::Accept,
};
use poem_openapi::{
    payload::Json,
    payload::Binary,
    param::Path,
    OpenApi,
    OpenApiService,
    ApiResponse,
};

struct Api;

#[derive(ApiResponse)]
enum NotebookDetailResponse {
    #[oai(status = 200)]
    Json(Json<Notebook>),
    #[oai(status = 200)]
    Binary(Binary<Vec<u8>>),
}

fn path_to_response(path: std::path::PathBuf) -> Result<Binary<Vec<u8>>> {
    let output_file = File::open(path.clone())
        .with_context(|| format!("Failed to open rendered file {:?}", path))?;

    let mut bytes = Vec::new();
    let mut reader = BufReader::new(output_file);
    reader.read_to_end(&mut bytes)
          .with_context(|| format!("Failed to read rendered file {:?}", path))?;
    Ok(Binary(bytes))
}

#[OpenApi(prefix_path = "/api/v1")]
impl Api {
    #[oai(path = "/notebooks", method = "get")]
    async fn notenbooks_list(&self) -> Result<Json<Vec<Notebook>>> {
        let notebooks = notebooks::list_notebooks()
        .context("Failed to list notebooks")?;
        Ok(Json(notebooks))
    }

    #[oai(path = "/notebooks/:id", method = "get")]
    async fn notebook_detail(&self, accept: Accept, id: Path<String>) -> Result<NotebookDetailResponse> {
        let notebook = notebooks::get_notebook_by_id(id.to_string())
            .with_context(|| format!("Failed to get notebook {}", id.to_string()))?;

        for mime in &accept.0 {
            match mime.as_ref() {
                "application/pdf" | "application/octet-stream"
                => {
                    // FIXME tempfile
                    let output_path = "/home/phaer/src/remarkable/test.pdf";
                    notebook
                        .to_pdf(output_path)
                        .with_context(|| format!("Failed to render notebook {}", id.to_string()))?;
                    return Ok(NotebookDetailResponse::Binary(path_to_response(std::path::PathBuf::from(output_path))?));
                }
                "image/svg+xml" => {
                    // FIXME tempfile
                    let output_path = "/home/phaer/src/remarkable/test.svg";
                    let mut output_file = File::create(output_path)
                        .with_context(|| format!("Failed to create output file {}", output_path))?;
                    notebook
                        .to_svg(&mut output_file, 0)
                        .with_context(|| format!("Failed to render notebook {}", id.to_string()))?;
                    return Ok(NotebookDetailResponse::Binary(path_to_response(std::path::PathBuf::from(output_path))?));
                }
                _ => {}
            }
        }
        Ok(NotebookDetailResponse::Json(Json(notebook)))
    }
}

pub async fn start(addr: String) -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }

    let url = format!("http://{}", addr);
    let api_service = OpenApiService::new(Api, "Unremarkable", "0.1")
        .server(url.clone());

    let ui = api_service.swagger_ui();
    let spec = api_service.spec_endpoint_yaml();
    let app = Route::new()
        .nest("/", api_service)
        .nest("/docs", ui)
        .nest("/spec", spec);

    println!("Listening on {}", url);
    Server::new(TcpListener::bind(addr))
        .run(app)
        .await
}
