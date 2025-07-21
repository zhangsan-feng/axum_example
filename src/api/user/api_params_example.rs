use std::any::Any;

use std::fmt::{Debug, Display, Formatter};
use axum::extract::{Query, State};
use axum::{Form, Json};
use axum::http::{ StatusCode};
use axum::response::{IntoResponse, Response};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::utils::response_adapter::{RestTemplateError, RestTemplate};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserQueryParams {
    username:String,
    password:String,
}

pub async fn get_params_example(Query(params):Query<UserQueryParams>,)->  Result<RestTemplate<Value>, RestTemplateError>  {
    info!("username:{} password:{}", params.username, params.password);
    // let s = params.password.parse::<i32>().map_err(|_|AppError::ServiceError(String::from("弄错了 ")))?;
    let s = params.password.parse::<i32>()?;
    // let s = params.password.parse::<i32>().expect("conv error");
    Ok(RestTemplate::success(200, "".parse()?))
}

