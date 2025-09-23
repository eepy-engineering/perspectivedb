use core::ops::DerefMut;

use crate::util::{
    buffer::Buffer,
    growable::Growable,
    tree::{ArenaTree, Node},
};

pub enum FilterLiteral {
    String(usize),
    Number(f64),
    Boolean(bool),
}

pub enum FilterOperation {
    Add,
    Sub,
    Mul,
    Div,

    Custom(usize),
}

pub enum FilterNode {
    Literal(FilterLiteral),
    Operation(FilterOperation),
    Reference(usize),
}

pub struct Filter<
    KeyA: DerefMut<Target = [u8]>,
    TreeA: DerefMut<Target = [Option<Node<FilterNode>>]>,
> {
    buffer: Buffer<KeyA>,
    tree: ArenaTree<FilterNode, TreeA>,
}

impl<KeyA: DerefMut<Target = [u8]>, TreeA: DerefMut<Target = [Option<Node<FilterNode>>]>>
    Filter<KeyA, TreeA>
{
    pub fn new(key_arena: KeyA, tree_arena: TreeA) -> Self {
        Filter {
            buffer: Buffer::new(key_arena),
            tree: ArenaTree::new(tree_arena),
        }
    }

    pub fn new_growable(key_arena: KeyA, tree_arena: TreeA) -> Self
    where
        KeyA: Growable,
        TreeA: Growable,
    {
        Filter {
            buffer: Buffer::new_growable(key_arena),
            tree: ArenaTree::new_growable(tree_arena),
        }
    }
}

// impl<
//     KeyA1: DerefMut<Target = [u8]>,
//     TreeA1: DerefMut<Target = [Option<Node<FilterNode>>]>,
//     KeyA2: DerefMut<Target = [u8]>,
//     TreeA2: DerefMut<Target = [Option<Node<FilterNode>>]>,
// > PartialEq<Filter<KeyA2, TreeA2>> for Filter<KeyA1, TreeA1>
// {
//     fn eq(&self, rhs: &Filter<KeyA2, TreeA2>) -> bool {
//         let lhs = self.tree.root();
//         let rhs = rhs.tree.root();

//         if lhs.is_none() != rhs.is_none() {
//             return false;
//         }

//         let Some(lhs) = lhs else { return true };
//         let Some(rhs) = rhs else { return true };

//         lhs.children().zip(rhs.children())
//     }
// }
