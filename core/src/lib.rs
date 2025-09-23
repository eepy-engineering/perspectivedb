#![no_std]

use crate::persistence::Persistence;

pub mod config;
pub mod log;
pub mod persistence;
pub mod query;
pub mod util;

pub struct Database<P: Persistence> {
    persistence: P,
}

impl<P: Persistence> Database<P> {
    pub fn open(persistence: P) -> Self {
        Database { persistence }
    }
}
