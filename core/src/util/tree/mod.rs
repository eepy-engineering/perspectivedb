pub mod node_mut;
pub mod node_ref;

use core::ops::DerefMut;

use crate::util::{
    growable::Growable,
    tree::{node_mut::NodeMutRef, node_ref::NodeRef},
};

pub struct Node<T> {
    value: T,
    parent: usize,
    first_child: Option<usize>,
    next_sibling: Option<usize>,
}

pub struct ArenaTree<T, A: DerefMut<Target = [Option<Node<T>>]>> {
    arena: A,
    arena_grow: Option<fn(&mut A, assert_size: usize) -> ()>,
    root: Option<usize>,
}

impl<T, A: DerefMut<Target = [Option<Node<T>>]>> ArenaTree<T, A> {
    pub fn new(arena: A) -> Self {
        ArenaTree {
            arena: arena,
            arena_grow: None,
            root: None,
        }
    }
}

impl<T, A: DerefMut<Target = [Option<Node<T>>]> + Growable> ArenaTree<T, A> {
    pub fn new_growable(arena: A) -> Self {
        ArenaTree {
            arena: arena,
            arena_grow: Some(|arena, size| arena.assert_size(size)),
            root: None,
        }
    }
}

impl<T, A: DerefMut<Target = [Option<Node<T>>]>> ArenaTree<T, A> {
    fn resolve_mut(&mut self, index: usize) -> &mut Option<Node<T>> {
        &mut self.arena[index]
    }

    fn resolve(&self, index: usize) -> Option<&Node<T>> {
        self.arena[index].as_ref()
    }

    fn alloc(&mut self) -> Option<usize> {
        for (index, node) in self.arena.iter().enumerate() {
            if node.is_none() {
                return Some(index);
            }
        }

        if let Some(v) = self.arena_grow {
            let size = self.arena.len();

            (v)(&mut self.arena, size * 2);

            return Some(size);
        }

        None
    }

    pub fn root<'a>(&'a self) -> Option<NodeRef<'a, T, A>> {
        self.root.and_then(|v| self.resolve(v)).map(|v| NodeRef {
            value: v,
            arena: self,
        })
    }

    pub fn mut_root<'a>(&'a mut self) -> Option<NodeMutRef<'a, T, A>> {
        self.root.map(|v| NodeMutRef {
            value: v,
            arena: self,
        })
    }
}
