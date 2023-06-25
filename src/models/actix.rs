use std::sync::{Mutex, Arc};

pub struct AppConnection {
    pub redis: Arc<Mutex<redis::Connection>>,
    pub mongo: Arc<Mutex<mongodb::Database>>,
}