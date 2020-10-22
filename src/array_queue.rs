use std::{cmp, iter};

use crate::Queue;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub struct ArrayQueue<T> {
    a: Box<[Option<T>]>,
    j: usize, // current position
    n: usize, // the number of elements in the queue
}

impl<T> ArrayQueue<T> {
    fn alloc_boxed_slice(size: usize) -> Box<[Option<T>]> {
        iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
    pub fn new(len: usize) -> Self {
        Self {
            a: Self::alloc_boxed_slice(len),
            j: 0,
            n: 0,
        }
    }
    pub fn resize(&mut self) {
        let mut b = Self::alloc_boxed_slice(cmp::max(self.n * 2, 1));
        for k in 0..self.n {
            b[k] = self.a[(self.j + k) % self.a.len()].take();
        }
        self.a = b;
        self.j = 0;
    }
    pub fn debug_array_stack(&self) {
        println!("j: {}, n: {}", &self.j, &self.n);
    }
}

impl<T: Clone> Queue<T> for ArrayQueue<T> {
    fn add(&mut self, x: T) {
        if self.n + 1 >= self.a.len() {
            self.resize();
        }
        self.a[(self.j + self.n) % self.a.len()] = Some(x);
        self.n += 1;
    }
    fn remove(&mut self) -> Option<T> {
        let x = self.a[self.j].take();
        self.j = (self.j + 1) % self.a.len();
        self.n -= 1;
        if self.a.len() >= 3 * self.n {
            self.resize();
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayQueue;
    use crate::Queue;

    #[test]
    fn test_array_queue() {
        // initialize
        let mut q: ArrayQueue<i32> = ArrayQueue::new(10);
        check_queue_size(&q, 10, 0);
        // add and remove
        q.add(-1);
        q.add(1);
        for elem in [-1, 1].iter() {
            assert_eq!(q.remove(), Some(*elem));
        }
        check_queue_size(&q, 1, 0);
    }
    fn check_queue_size<T>(queue: &ArrayQueue<T>, len: usize, n: usize) {
        assert_eq!((queue.a.len(), queue.n), (len, n));
    }
}
