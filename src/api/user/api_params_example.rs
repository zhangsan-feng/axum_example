
use std::fmt::{Debug, Formatter};
use axum::extract::{Query, State};
use axum::{Json};
use axum::response::{ Response};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::utils::request_adapter::{ValidParams, ValidMultipartParams};
use crate::utils::response_adapter::{RestTemplateError, RestTemplate};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Params {
    username:String,
    password:String,
}

pub async fn get_params_example(ValidParams(params):ValidParams<Params>,)->  Result<RestTemplate<Value>, RestTemplateError>  {
    info!("username:{} password:{}", params.username, params.password);
    // let s = params.password.parse::<i32>().map_err(|_|RestTemplateError::ServiceError(String::from("弄错了 ")))?;
    // let s = params.password.parse::<i32>()?;
    Ok(RestTemplate::success(200, json!("")))
}

pub async fn post_json_params_example(ValidParams(params):ValidParams<Params>,)->  Result<RestTemplate<Value>, RestTemplateError>  {
    Ok(RestTemplate::success(200, json!("")))
}


pub async fn post_from_params_example(ValidParams(params):ValidParams<Params>,)->  Result<RestTemplate<Value>, RestTemplateError>  {
    Ok(RestTemplate::success(200, json!("")))
}



pub async fn post_file_example(ValidMultipartParams(data): ValidMultipartParams<Params>)->  Result<RestTemplate<Value>, RestTemplateError>  {
        
        
        Ok(RestTemplate::success(200, json!("")))
}
