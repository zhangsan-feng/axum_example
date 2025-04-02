use std::any::Any;

use std::fmt::{Debug, Display, Formatter};
use axum::extract::{Query, State};
use axum::{Form, Json};
use axum::http::{ StatusCode};
use axum::response::{IntoResponse, Response};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::{Error as StdError, Error};
use std::{fmt, io};
use std::ops::Deref;
use anyhow::Context;
use tracing::instrument::WithSubscriber;
use axum::extract::{DefaultBodyLimit, Multipart};
use serde::de::Unexpected::Str;
use tracing::{debug, Instrument};
use backtrace::Backtrace;
use walkdir::WalkDir;
use crate::middle::middle_error::AppError;

#[derive(Debug, Serialize, Deserialize, Clone,)]
pub struct UserEntity {
    username:String,
    password:String,
}

pub async fn user_info(Query(params):Query<UserEntity>,)->  Result<Json<Value>, AppError>  {
    info!("username:{} password:{}", params.username, params.password);
    // RestResponse::success();
    // let s = params.password.parse::<i32>().map_err(|_|AppError::ServiceError(String::from("弄错了 ")))?;
    let s = params.password.parse::<i32>()?;
    // let s = params.password.parse::<i32>().expect("conv error");
    Ok(ResponseStruct::success().await)
}

trait ResponseImpl {
    async fn success() -> Json<Value>;
    async fn failed() -> Json<Value>;
}

struct ResponseStruct;

impl ResponseImpl for ResponseStruct {
    async fn success() -> Json<Value> {
        Json(json!({
            "code": 200,
            "data": "",
            "message": ""
        }))
    }

    async fn failed() -> Json<Value> {
        Json(json!({
            "code": 500,
            "data": "",
            "message": ""
        }))
    }
}




