use std::{cmp, iter};

use crate::{array_stack::ArrayStack, List};
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RootishArrayStack<T> {
    blocks: Box<[ArrayStack<T>]>,
    n: usize,
}
impl<T> RootishArrayStack<T> {
    /// index to block
    fn i2b(&self, i: usize) -> usize {
        (-3.0 + (9.0 + 8.0 * (i as f32)).sqrt() / 2.0).ceil() as usize
    }
}

impl<T: Clone> List<T> for RootishArrayStack<T> {
    fn get(&self, i: usize) -> Option<T> {
        let b = self.i2b(i);
        let j = i - b * (b + 1) / 2;
        self.blocks[b].get(j)
    }
    #[allow(clippy::many_single_char_names)]
    fn set(&mut self, i: usize, x: T) -> Option<T> {
        let b = self.i2b(i);
        let j = i - b * (b + 1) / 2;
        let y = self.blocks[b].get(j);
        self.blocks[b].set(j, x);
        y
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn add(&mut self, i: usize, x: T) {
        todo!()
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_rootish_array_stack() {
        unimplemented!();
    }
}