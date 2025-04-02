use serde::{Deserialize, Serialize};
use axum::Form;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEditParams {
    id:i32,
    pic:String,
}


pub async fn user_edit(Form(form): Form<UserEditParams>){}