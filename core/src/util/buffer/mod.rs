use core::ops::DerefMut;

use crate::util::growable::Growable;

pub struct Buffer<A: DerefMut<Target = [u8]>> {
    arena: A,
    arena_grow: Option<fn(&mut A, assert_size: usize) -> ()>,
    head: usize,
}

impl<A: DerefMut<Target = [u8]>> Buffer<A> {
    pub fn new(arena: A) -> Self {
        Buffer {
            arena: arena,
            arena_grow: None,
            head: 0,
        }
    }
}

impl<A: DerefMut<Target = [u8]> + Growable> Buffer<A> {
    pub fn new_growable(arena: A) -> Self {
        Buffer {
            arena: arena,
            arena_grow: Some(|arena, size| arena.assert_size(size)),
            head: 0,
        }
    }
}

impl<A: DerefMut<Target = [u8]>> Buffer<A> {
    pub fn push(&mut self, data: &[u8]) -> Result<(), ()> {
        if self.arena.len() - self.head < data.len() + 4 {
            let Some(arena_grow) = self.arena_grow else {
                return Err(());
            };

            let current_len = self.arena.len();
            arena_grow(&mut self.arena, current_len * 2);
        };

        for (idx, v) in data.len().to_ne_bytes().iter().enumerate() {
            self.arena[self.head + idx] = *v;
        }

        self.head += 4;

        for (idx, v) in data.iter().enumerate() {
            self.arena[self.head + idx] = *v
        }

        self.head += data.len();

        Ok(())
    }

    pub fn at(&self, index: usize) -> Option<&[u8]> {
        let mut offset = 0;

        for _ in 0..index {
            let distance = u32::from_ne_bytes([
                self.arena[offset],
                self.arena[offset + 1],
                self.arena[offset + 2],
                self.arena[offset + 3],
            ]);

            offset += distance as usize;
        }

        let length = u32::from_ne_bytes([
            self.arena[offset],
            self.arena[offset + 1],
            self.arena[offset + 2],
            self.arena[offset + 3],
        ]);

        Some(&self.arena[offset..(offset + length as usize)])
    }
}
