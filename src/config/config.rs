

use std::sync::{LazyLock, OnceLock};
use tokio::sync::{OnceCell};

static CONN_LAZY_LOCK: LazyLock<String> = LazyLock::new(|| { String::from("conn")});
static CONN_ONCE_LOCK: OnceLock<String> = OnceLock::new();
static CONN_ONCE_CELL: OnceCell<String> = OnceCell::const_new();




// https://course.rs/basic/variable.html
// Cell RefCell 
// Fn FnMut FnOnce  
// Box Rc Arc 
// Future
// channel 

