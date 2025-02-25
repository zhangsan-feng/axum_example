use crate::config;

pub async fn config_init(){
    const LOGGER_PATH: &str = "./logs/";
    config::logger::logger_init(LOGGER_PATH).await;
}