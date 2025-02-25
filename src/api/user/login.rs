use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoginParams {
    username:String,
    password:String,
}

pub async fn user_login(Json(params):Json<LoginParams>){

}