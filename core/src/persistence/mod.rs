use core::ops::DerefMut;

use crate::{
    config::DatabaseConfig,
    log::identifier::EntryId,
    query::{Query, QueryAny, filter::FilterNode},
    util::tree::Node,
};

#[derive(Clone, Copy, PartialEq)]
pub enum LogPosition {
    Head,
    Tail,
    Before(EntryId),
    After(EntryId),
}

pub trait Persistence {
    type Table: Table;

    fn config(&self) -> &DatabaseConfig;
    fn table(&self, table: &str) -> Option<Self::Table>;
}

pub trait Table {
    type Log: Log;
    type ResultCache: ResultCache;

    fn log(&self) -> &Self::Log;

    fn query<KeyA: DerefMut<Target = [u8]>, TreeA: DerefMut<Target = [Option<Node<FilterNode>>]>>(
        &self,
        query: Query<KeyA, TreeA>,
    ) -> Option<Self::ResultCache>;

    fn queries<'a>(&'a self) -> impl Iterator<Item = QueryAny<'a>>;
}

pub trait ResultCache {}

pub trait Log {
    type Error;

    fn append(&mut self, data: &[u8]) -> impl Future<Output = Result<EntryId, Self::Error>>;

    fn insert(
        &mut self,
        at: LogPosition,
        data: &[u8],
    ) -> impl Future<Output = Result<EntryId, Self::Error>>;

    type Iterator<'a>: Iterator<Item = &'a [u8]>
    where
        Self: 'a;

    fn iter<'a>(
        &'a mut self,
        start: LogPosition,
        end: LogPosition,
    ) -> impl Future<Output = Result<Self::Iterator<'a>, Self::Error>>;
}
