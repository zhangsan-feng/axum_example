use std::collections::HashMap;
use axum::extract::Query;



pub async fn user_detail(Query(params): Query<HashMap<String, String>>){}