use core::ops::{Deref, DerefMut};

use crate::util::tree::{
    ArenaTree, Node,
    node_ref::{ChildrenIterator, NodeRef},
};

pub struct NodeMutRef<'a, T, A: DerefMut<Target = [Option<Node<T>>]>> {
    pub(super) value: usize,
    pub(super) arena: &'a mut ArenaTree<T, A>,
}

pub struct ChildrenMutIterator<'a, T, A: DerefMut<Target = [Option<Node<T>>]>> {
    reference: Option<NodeMutRef<'a, T, A>>,
}

impl<'a, T, A: DerefMut<Target = [Option<Node<T>>]>> NodeMutRef<'a, T, A> {
    pub fn children(&'a self) -> ChildrenIterator<'a, T, A> {
        let node = self
            .arena
            .resolve(self.value)
            .expect("Pointer dereference failed in tree. Dangling?");

        let Some(v) = node.first_child else {
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

    pub fn children_mut(&'a mut self) -> ChildrenMutIterator<'a, T, A> {
        let node = self
            .arena
            .resolve(self.value)
            .expect("Pointer dereference failed in tree. Dangling?");

        let Some(v) = node.first_child else {
            return ChildrenMutIterator { reference: None };
        };

        return ChildrenMutIterator {
            reference: Some(NodeMutRef {
                value: v,
                arena: self.arena,
            }),
        };
    }

    pub fn append_child(&mut self, value: T) -> Result<(), ()> {
        let node = self.arena.resolve(self.value).unwrap();

        let Some(mut child) = node.first_child else {
            let Some(new_index) = self.arena.alloc() else {
                return Err(());
            };

            *self.arena.resolve_mut(new_index) = Some(Node {
                value,
                parent: self.value,
                first_child: None,
                next_sibling: None,
            });

            let node = self.arena.resolve_mut(self.value).as_mut().unwrap();
            node.first_child = Some(new_index);

            return Ok(());
        };

        loop {
            let n2 = self.arena.resolve(child).unwrap();

            if let Some(v) = n2.next_sibling {
                child = v;
                continue;
            }

            let Some(new_index) = self.arena.alloc() else {
                return Err(());
            };

            *self.arena.resolve_mut(new_index) = Some(Node {
                value,
                parent: self.value,
                first_child: None,
                next_sibling: None,
            });

            let node = self.arena.resolve_mut(self.value).as_mut().unwrap();
            node.first_child = Some(new_index);

            return Ok(());
        }
    }
}

impl<'a, T, A: DerefMut<Target = [Option<Node<T>>]>> Deref for NodeMutRef<'a, T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let node = self
            .arena
            .resolve(self.value)
            .expect("Pointer dereference failed in tree. Dangling?");

        &node.value
    }
}

impl<'a, T, A: DerefMut<Target = [Option<Node<T>>]>> DerefMut for NodeMutRef<'a, T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let node = self
            .arena
            .resolve_mut(self.value)
            .as_mut()
            .expect("Pointer dereference failed in tree. Dangling?");

        &mut node.value
    }
}
