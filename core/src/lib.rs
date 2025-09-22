use crate::config::DatabaseConfig;

pub mod config;
pub mod log;
pub mod persistence;
pub mod util;

pub struct Database {
    config: DatabaseConfig,
}

impl Database {
    pub fn new() {}
}
