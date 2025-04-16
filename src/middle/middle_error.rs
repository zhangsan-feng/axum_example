use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use log::info;
use http_body_util::BodyExt;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    ServiceError(String),
    DBError(String),
    WithCodeMsg(i32, String),

}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ServiceError(msg) => (StatusCode::BAD_GATEWAY, msg),
            AppError::DBError(msg) => (StatusCode::BAD_GATEWAY, msg),
            _ => (StatusCode::BAD_GATEWAY, "".to_string()),
        };

        (status, error_message).into_response()
        // (status, Json()).into_response()
    }
}


impl<E> From<E> for AppError where E: Into<anyhow::Error>  {
    fn from(err: E) -> Self {
        let err_source: anyhow::Error = err.into();
        let src_files: Vec<String> = WalkDir::new(env!("CARGO_MANIFEST_DIR").to_owned() + "/src")
            .into_iter().filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().is_file() {
                let path = entry.path();
                if path.extension()? == "rs" && path.file_name()? != "mod.rs"{
                    path.file_name()?.to_string_lossy().into_owned().into()
                } else {
                    None
                }
            } else {
                None
            }
        })
            .collect();

        let backtrace = err_source.backtrace().to_string().lines()
            .filter(|l| !l.contains("mod.rs"))
            .filter(|l| src_files.iter().any(|file| l.replace("\\","/").contains(file))).
            collect::<Vec<_>>().join("\n");

        info!("\n{}\n Error:{}", backtrace , err_source.to_string());

        AppError::ServiceError(err_source.to_string())
    }
}


