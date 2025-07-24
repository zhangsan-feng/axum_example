use std::collections::HashMap;
use axum::body::Bytes;
use axum::extract::{FromRequest, Multipart, Request};
use axum::http::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::utils::response_adapter::RestTemplateError;

pub struct ValidParams<T>(pub T);
impl<T, S> FromRequest<S> for ValidParams<T>
where
    T: Serialize + DeserializeOwned  + Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = RestTemplateError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let method = req.method().clone();
        let uri = req.uri().clone();
        let headers = req.headers().clone();

        match method {
            Method::GET  => {
                // 对于 GET 请求，从查询参数解析
                let query_string = uri.query().unwrap_or("");

                if query_string.is_empty() {
                    return Err(RestTemplateError::ServiceError("缺少查询参数".to_string()));
                }

                // 将查询字符串转换为 serde_urlencoded 可以解析的格式
                let params: T = serde_urlencoded::from_str(query_string)
                    .map_err(|e| {
                        RestTemplateError::ServiceError(format!("查询参数解析失败: {}", e))
                    })?;

                Ok(ValidParams(params))
            }
            Method::POST =>{
                // 获取 Content-Type
                let content_type = headers.get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("");

                // 读取请求体
                let bytes = Bytes::from_request(req, state).await
                    .map_err(|e| RestTemplateError::ServiceError(format!("读取请求体失败: {}", e)))?;

                if bytes.is_empty() {
                    return Err(RestTemplateError::ServiceError("请求体为空".to_string()));
                }

                // 根据 Content-Type 选择解析方式
                if content_type.contains("application/x-www-form-urlencoded") {
                    // 处理 form 参数
                    let form_str = std::str::from_utf8(&bytes)
                        .map_err(|e| RestTemplateError::ServiceError(format!("请求体编码错误: {}", e)))?;

                    let params: T = serde_urlencoded::from_str(form_str)
                        .map_err(|e| {
                            RestTemplateError::ServiceError(format!("Form 参数解析失败: {}", e))
                        })?;

                    Ok(ValidParams(params))
                } else if content_type.contains("application/json") {
                    // 默认按 JSON 处理
                    let params: T = serde_json::from_slice(&bytes)
                        .map_err(|e| {
                            RestTemplateError::ServiceError(format!("JSON 参数解析失败: {}", e))
                        })?;

                    Ok(ValidParams(params))
                }else if content_type.contains("multipart/form-data") {
                    Err(RestTemplateError::ServiceError(format!("不支持得请求头: {}", content_type)))
                }

                else {
                    Err(RestTemplateError::ServiceError(format!("不支持得请求头: {}", content_type)))
                }
            }
            _ => {
                Err(RestTemplateError::ServiceError(format!("不支持的请求方法: {}", method)))
            }
        }

    }

}


#[derive(Debug, Clone)]
pub struct FileData {
    pub field_name: String,
    pub file_name: Option<String>,
    pub content_type: Option<String>,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct MultipartData<T> {
    pub params: T,
    pub files: Vec<FileData>,
}

pub struct ValidMultipartParams<T>(pub MultipartData<T>);

impl<T, S> FromRequest<S> for ValidMultipartParams<T>
where
    T: DeserializeOwned + Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = RestTemplateError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req, state)
            .await
            .map_err(|e| RestTemplateError::ServiceError(format!("解析 multipart 失败: {}", e)))?;

        let mut form_params: HashMap<String, String> = HashMap::new();
        let mut files: Vec<FileData> = Vec::new();

        while let Some(field) = multipart.next_field().await.map_err(|e| {
            RestTemplateError::ServiceError(format!("读取 multipart 字段失败: {}", e))
        })? {
            let field_name = field.name().unwrap_or("unknown").to_string();
            let file_name = field.file_name().map(|s| s.to_string());
            let content_type = field.content_type().map(|s| s.to_string());

            let data = field.bytes().await.map_err(|e| { RestTemplateError::ServiceError(format!("读取字段数据失败: {}", e)) })?;

            if file_name.is_some() || content_type.is_some() {
        
                files.push(FileData {
                    field_name: field_name.clone(),
                    file_name,
                    content_type,
                    data: data.to_vec(),
                });
            } else {
       
                let value = String::from_utf8(data.to_vec())
                    .map_err(|e| RestTemplateError::ServiceError(format!("参数编码错误: {}", e)))?;
                form_params.insert(field_name, value);
            }
        }

        // 将表单参数转换为结构体
        let params_string = serde_urlencoded::to_string(&form_params)
            .map_err(|e| RestTemplateError::ServiceError(format!("参数序列化失败: {}", e)))?;

        let params: T = serde_urlencoded::from_str(&params_string)
            .map_err(|e| RestTemplateError::ServiceError(format!("参数解析失败: {}", e)))?;

        Ok(ValidMultipartParams(MultipartData { params, files }))
    }
}
