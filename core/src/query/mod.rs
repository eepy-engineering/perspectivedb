use core::ops::DerefMut;

use crate::{
    query::filter::{Filter, FilterNode},
    util::tree::Node,
};

pub mod filter;
pub mod sort;

pub struct Query<
    KeyA: DerefMut<Target = [u8]>,
    TreeA: DerefMut<Target = [Option<Node<FilterNode>>]>,
> {
    filter: Filter<KeyA, TreeA>,
    // sort: ,
}
