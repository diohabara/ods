use std::{cell::RefCell, rc::Rc};

use crate::{array_stack::ArrayStack, List};
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RootishArrayStack<T> {
    blocks: ArrayStack<Rc<[RefCell<Option<T>>]>>,
    n: usize,
}

impl<T: Clone> Default for RootishArrayStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> RootishArrayStack<T> {
    pub fn new() -> Self {
        Self {
            blocks: ArrayStack::new(1),
            n: 0,
        }
    }
    /// index to block
    fn i2b(&self, i: usize) -> usize {
        ((-3.0 + (9.0 + 8.0 * i as f64).sqrt()) / 2f64).ceil() as usize
    }
    fn grow(&mut self) {
        let block = std::iter::repeat_with(Default::default)
            .take(self.blocks.size() + 1)
            .collect::<Rc<_>>();
        self.blocks.add(self.blocks.size(), block);
    }
    fn shrink(&mut self) {
        let mut r = self.blocks.size();
        while 0 < r && self.n <= (std::cmp::max(2, r) - 2) * (r - 1) / 2 {
            self.blocks.remove(self.blocks.size() - 1);
            r -= 1;
        }
    }
}

impl<T: Clone> List<T> for RootishArrayStack<T> {
    fn size(&self) -> usize {
        self.n
    }
    fn get(&self, i: usize) -> Option<T> {
        let b = self.i2b(i);
        let j = i - b * (b + 1) / 2;
        match self.blocks.get(b)?[j].borrow().as_ref() {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }
    #[allow(clippy::many_single_char_names)]
    fn set(&mut self, i: usize, x: T) -> Option<T> {
        let b = self.i2b(i);
        let j = i - b * (b + 1) / 2;
        self.blocks.get(b)?[j].borrow_mut().replace(x)
    }
    fn add(&mut self, i: usize, x: T) {
        assert!(i <= self.n);
        let r = self.blocks.size();
        if r * (r + 1) / 2 < self.n + 1 {
            self.grow();
        }
        self.n += 1;
        for j in (i + 1..self.n).rev() {
            self.set(j, self.get(j - 1).unwrap());
        }
        self.set(i, x);
    }
    fn remove(&mut self, i: usize) -> Option<T> {
        if i < self.n {
            let x = self.get(i);
            for j in i..self.n - 1 {
                self.set(j, self.get(j + 1).unwrap());
            }
            let eb = self.i2b(self.n - 1);
            let ej = self.n - 1 - eb * (eb + 1) / 2;
            self.blocks.get(eb)?[ej].borrow_mut().take();
            self.n -= 1;
            let r = self.blocks.size();
            if (r - 2) * (r - 1) / 2 <= self.n {
                self.shrink();
            }
            x
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RootishArrayStack;
    use crate::List;

    #[test]
    fn test_rootish_array_stack() {
        let mut rootish_array_stack: RootishArrayStack<char> = RootishArrayStack::new();
        assert_eq!(rootish_array_stack.size(), 0);
        for (i, elem) in "abcde".chars().enumerate() {
            rootish_array_stack.add(i, elem);
        }
        for (i, elem) in "abcde".chars().enumerate() {
            assert_eq!(rootish_array_stack.get(i), Some(elem));
        }
        rootish_array_stack.add(2, 'x'); // "abxcde"
        rootish_array_stack.remove(1); // "axcde"
        for (i, elem) in "axcde".chars().enumerate() {
            assert_eq!(rootish_array_stack.get(i), Some(elem));
        }
        let mut rootish_array_stack: RootishArrayStack<i32> = RootishArrayStack::new();
        let num = 10;
        for i in 0..num {
            rootish_array_stack.add(rootish_array_stack.size(), i);
        }
        while rootish_array_stack.remove(0).is_some() {}
    }
}
