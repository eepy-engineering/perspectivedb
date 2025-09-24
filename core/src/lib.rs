#![no_std]

use crate::{persistence::Persistence, table::Table};

pub mod config;
pub mod log;
pub mod persistence;
pub mod query;
pub mod table;
pub mod util;

pub struct Database<P: Persistence> {
    persistence: P,
}

impl<P: Persistence> Database<P> {
    pub fn open(persistence: P) -> Self {
        Database { persistence }
    }

    pub fn table<'a>(&'a self, table_name: &str) -> Option<Table<'a, P>> {
        self.persistence.table(table_name).map(Table::new)
    }
}
