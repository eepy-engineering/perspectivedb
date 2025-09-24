use crate::persistence::{self, Persistence};

pub struct Table<'a, P: Persistence> {
    persistence: &'a P::Table,
}

impl<'a, P: Persistence> Table<'a, P> {
    pub(crate) fn new(persistence: &'a P::Table) -> Self {
        Self { persistence }
    }
}
