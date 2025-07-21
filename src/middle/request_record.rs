use std::collections::HashMap;
use axum::body::{Body, Bytes};
use axum::BoxError;
use axum::extract::Request;
use axum::http::{HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Response};
use http_body_util::BodyExt;
use log::debug;
use log::info;
use serde_json::{json, Value};

pub async  fn request_record(req: Request, next: Next) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = body.collect().await.expect("collect request body error").to_bytes();
    let req = Request::from_parts(parts, Body::from(bytes.clone()));

    // let request_headers:HashMap<String, String> = req.headers().iter()
    //     .filter_map(|(name, value)| {
    //         value.to_str().ok().map(|v| (name.to_string(), v.to_string()))
    //     })
    //     .collect();
    //
    // info!("{}",serde_json::to_string_pretty(&request_headers).unwrap_or_default());

    // match String::from_utf8(request_body_bytes.to_vec()) {
    //     Ok(body_string) => {
    //         // 尝试将字符串解析为 JSON
    //         match serde_json::from_str::<serde_json::Value>(&body_string) {
    //             Ok(json_value) => {
    //                 // 如果是有效的 JSON，则美化打印
    //                 info!("{}", serde_json::to_string_pretty(&json_value).unwrap_or_default());
    //             },
    //             Err(_) => {
    //                 // 如果不是有效的 JSON，则打印原始字符串
    //                 info!("Request body (not JSON): {}", body_string);
    //             }
    //         }
    //     },
    //     Err(_) => {
    //         // 如果不是有效的 UTF-8 字符串，则打印原始字节的调试表示（例如，十六进制）
    //         // 这种情况下通常是二进制数据（如图片、文件），直接打印字符串会是乱码
    //         info!("Request body (binary/non-UTF8): {:?}", request_body_bytes);
    //     }
    // }
    
    // info!("{:?}", req.body());

    let res = next.run(req).await;
    let (parts, body) = res.into_parts();
    let bytes = body.collect().await.expect("collect response body error").to_bytes();
    let res = Response::from_parts(parts, Body::from(bytes.clone()));
    // info!("{:?}", res);
    Ok(res)
}

