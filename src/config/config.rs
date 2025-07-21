use crate::config;

use std::sync::{LazyLock, OnceLock};
use lazy_static::lazy_static;
use once_cell::sync::{Lazy, OnceCell};
use sea_orm::{Database, DbConn};
use tokio::runtime::Builder;


lazy_static!(static ref MYSQL:&'static str = "";);
static ONCE_CELL_EXAMPLE: OnceCell<String> = OnceCell::new();
static ONCE_LOCK_EXAMPLE: OnceLock<Vec<i32>> = OnceLock::new();
static LAZY_LOCK_EXAMPLE: LazyLock<Vec<i32>> = LazyLock::new(|| { vec![1, 2, 3] });
static MYSQL_CONN: Lazy<&'static str> = Lazy::new(|| {""});

