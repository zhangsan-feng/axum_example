use std::env;
use axum::{http, Json};
use axum::extract::{ Request, Multipart};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use log::info;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RestTemplate<T> {
    code:i32,
    msg:String,
    data:Option<T>
}


impl<T> IntoResponse for RestTemplate<T> where T: Serialize {
    fn into_response(self) -> Response {
        (http::StatusCode::OK, Json(self)).into_response()
    }
}

impl <T>RestTemplate<T> where T: Serialize {
    pub fn success(code:i32, data:T) -> Self {
        RestTemplate{
            code,
            msg: "".to_string(),
            data: Some(data),
        }
    }
    
    pub fn error(code:i32, data:T) -> Self {
        RestTemplate{
            code,
            msg: "".to_string(),
            data: Some(data),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestTemplateError {
    ServiceError(String),
    DBError(String),
    WithCodeMsg(i32, String),

}

impl IntoResponse for RestTemplateError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            RestTemplateError::ServiceError(msg) => (StatusCode::BAD_GATEWAY, msg),
            RestTemplateError::DBError(msg) => (StatusCode::BAD_GATEWAY, msg),
            _ => (StatusCode::BAD_GATEWAY, "".to_string()),
        };

        (status, error_message).into_response()
        // (status, Json()).into_response()
    }
}


impl<T> From<T> for RestTemplateError where T: Into<anyhow::Error>  {
    fn from(err: T) -> Self {
        let err_source: anyhow::Error = err.into();
        let src_files: Vec<String> = WalkDir::new(env!("CARGO_MANIFEST_DIR").to_string() + "/src")
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
        }).collect();

        let backtrace = err_source.backtrace().to_string().lines()
            .filter(|l| !l.contains("mod.rs"))
            .filter(|l| src_files.iter().any(|file| l.replace("\\","/").contains(file))).
            collect::<Vec<_>>().join("\n");

        info!("\n{}\n Error:{}", backtrace , err_source.to_string());

        RestTemplateError::ServiceError(err_source.to_string())
    }
}
