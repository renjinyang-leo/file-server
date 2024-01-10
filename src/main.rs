use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

use uuid::Uuid;

use axum::{
    extract::{ContentLengthLimit, Multipart},
    http::header::HeaderMap,
    http::StatusCode,
    routing::post,
    Router,
};

const SAVE_FILE_BASE_PATH: &str = "/Users/file";

async fn upload_file(
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            1024 * 1024 * 20 //20M
        },
    >,
) -> Result<(StatusCode, HeaderMap), String> {
    if let Some(file) = multipart.next_field().await.unwrap() {
        let content_type = file.content_type().unwrap().to_string();
 
        let id =  Uuid::new_v4();
        
        let index = content_type
            .find(".")
            .map(|i| i)
            .unwrap_or(usize::max_value());

        if index == usize::max_value() {
            return Err("parse file type failed".to_string())
        }
        let file_type = &content_type[index + 1..];

        let save_filename = format!("{}/{}.{}", SAVE_FILE_BASE_PATH, id, file_type);
        let data = file.bytes().await.unwrap();

        tokio::fs::write(&save_filename, &data)
            .await
            .map_err(|err| err.to_string())?;

        return Ok((StatusCode::OK, HeaderMap::new()))
    }
    Err("some error happen!".to_string())
}

#[tokio::main]
async fn main() {
    // our router
    let app = Router::new()
        .route("/upload", post(upload_file))
        .layer(TraceLayer::new_for_http());
 
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
 
    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
