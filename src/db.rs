use std::sync::{Arc, Mutex};
use crate::models::FabricOrder;

pub type Db = Arc<Mutex<Vec<FabricOrder>>>;

pub fn init_db() -> Db {
    Arc::new(Mutex::new(Vec::new()))
}
