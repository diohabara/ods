use std::{cmp, iter};

use crate::List;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ArrayDeque<T> {
    a: Box<[Option<T>]>,
    j: usize, // current position
    n: usize, // the number of elements in the deque
}

impl<T> ArrayDeque<T> {
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
    #[allow(dead_code)]
    pub fn debug_array_stack(&self) {
        println!("j: {}, n: {}", &self.j, &self.n);
    }
}

impl<T: Clone> List<T> for ArrayDeque<T> {
    fn size(&self) -> usize {
        self.n
    }
    fn get(&self, i: usize) -> Option<T> {
        self.a[(self.j + i) % self.a.len()].clone()
    }
    fn set(&mut self, i: usize, x: T) -> Option<T> {
        let y = self.a[(self.j + i) % self.a.len()].take();
        self.a[(self.j + i) % self.a.len()] = Some(x);
        y
    }
    fn add(&mut self, i: usize, x: T) {
        if self.n + 1 >= self.a.len() {
            self.resize();
        }
        if i < self.n / 2 {
            self.j = if self.j == 0 {
                self.a.len() - 1
            } else {
                self.j - 1
            };
            for k in 0..i {
                self.a[(self.j + k) % self.a.len()] =
                    self.a[(self.j + k + 1) % self.a.len()].take();
            }
        } else {
            for k in (i + 1..=self.n).rev() {
                self.a[(self.j + k) % self.a.len()] =
                    self.a[(self.j + k - 1) % self.a.len()].take();
            }
        }
        self.a[(self.j + i) % self.a.len()] = Some(x);
        self.n += 1;
    }
    fn remove(&mut self, i: usize) -> Option<T> {
        let x = self.a[(self.j + i) % self.a.len()].take();
        if i < self.n / 2 {
            for k in (1..=i).rev() {
                self.a[(self.j + k) % self.a.len()] =
                    self.a[(self.j + k - 1) % self.a.len()].take();
            }
        } else {
            for k in i..self.n - 1 {
                self.a[(self.j + k) % self.a.len()] =
                    self.a[(self.j + k + 1) % self.a.len()].take();
            }
        }
        self.n -= 1;
        if 3 * self.n < self.a.len() {
            self.resize();
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayDeque;
    use crate::List;

    #[test]
    fn test_array_queue() {
        // initialize & add
        let mut deq: ArrayDeque<char> = ArrayDeque::new(10);
        check_deque_size(&deq, 10, 0);
        for c in "abcdef".chars() {
            deq.add(0, c);
        }
        check_deque_size(&deq, 10, 6);

        // set & get
        deq.set(3, 'c');
        let achieved = deq.get(3);
        let expected = Some('c');
        assert_eq!(achieved, expected);

        // remove
        let achieved = deq.remove(5);
        let expected = Some('a');
        assert_eq!(achieved, expected);
        let achieved = deq.remove(4);
        let expected = Some('b');
        assert_eq!(achieved, expected);
    }
    fn check_deque_size<T>(deque: &ArrayDeque<T>, len: usize, n: usize) {
        assert_eq!((deque.a.len(), deque.n), (len, n));
    }
}
