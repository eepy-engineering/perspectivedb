use core::ops::{Deref, DerefMut};

use crate::util::tree::{ArenaTree, Node};

pub struct NodeRef<'a, T, A: DerefMut<Target = [Option<Node<T>>]>> {
    pub(super) value: &'a Node<T>,
    pub(super) arena: &'a ArenaTree<T, A>,
}

impl<'a, T, A: DerefMut<Target = [Option<Node<T>>]>> Copy for NodeRef<'a, T, A> {}
impl<'a, T, A: DerefMut<Target = [Option<Node<T>>]>> Clone for NodeRef<'a, T, A> {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct ChildrenIterator<'a, T, A: DerefMut<Target = [Option<Node<T>>]>> {
    pub(super) reference: Option<NodeRef<'a, T, A>>,
}

impl<'a, T, A: DerefMut<Target = [Option<Node<T>>]>> NodeRef<'a, T, A> {
    pub fn children(&self) -> ChildrenIterator<'a, T, A> {
        let Some(v) = self.value.first_child else {
            return ChildrenIterator { reference: None };
        };

        let node = self
            .arena
            .resolve(v)
            .expect("Pointer dereference failed in tree. Dangling?");

        return ChildrenIterator {
            reference: Some(NodeRef {
                value: node,
                arena: self.arena,
            }),
        };
    }
}

impl<'a, T, A: DerefMut<Target = [Option<Node<T>>]>> Iterator for ChildrenIterator<'a, T, A> {
    type Item = NodeRef<'a, T, A>;

    fn next(&mut self) -> Option<Self::Item> {
        let refer = (&*self).reference?;

        let next = refer
            .value
            .next_sibling
            .and_then(|v| refer.arena.resolve(v));

        self.reference = next.map(|v| NodeRef {
            value: v,
            arena: refer.arena,
        });

        return Some(refer);
    }
}

impl<'a, T, A: DerefMut<Target = [Option<Node<T>>]>> Deref for NodeRef<'a, T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value.value
    }
}
