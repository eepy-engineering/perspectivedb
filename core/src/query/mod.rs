use core::ops::{Deref, DerefMut};

use crate::{
    query::filter::{Filter, FilterNode},
    util::tree::Node,
};

pub mod filter;
pub mod sort;

pub struct CollectorAny<'l, T: ?Sized> {
    v: &'l mut dyn DerefMut<Target = T>,
}

impl<'l, T: ?Sized> Deref for CollectorAny<'l, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.v.deref()
    }
}

impl<'l, T: ?Sized> DerefMut for CollectorAny<'l, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.v.deref_mut()
    }
}

pub struct QueryAny<'l> {
    underlying: Query<CollectorAny<'l, [u8]>, CollectorAny<'l, [Option<Node<FilterNode>>]>>,
}

pub struct Query<
    KeyA: DerefMut<Target = [u8]>,
    TreeA: DerefMut<Target = [Option<Node<FilterNode>>]>,
> {
    filter: Filter<KeyA, TreeA>,
    // sort: ,
}
