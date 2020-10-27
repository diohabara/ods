use std::{cmp, iter};

use crate::{array_stack::ArrayStack, List};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DualArrayDeque<T> {
    front: ArrayStack<T>,
    back: ArrayStack<T>,
}

impl<T: Clone> DualArrayDeque<T> {
    pub fn new() -> Self {
        Self {
            front: ArrayStack::<T>::new(0),
            back: ArrayStack::<T>::new(0),
        }
    }
    fn balance(&mut self) {
        if 3 * self.front.size() < self.back.size() || 3 * self.back.size() < self.front.size() {
            let n = self.front.size() + self.back.size();
            let nf = n / 2;
            let mut af: Box<[Option<T>]> = iter::repeat_with(Default::default)
                .take(cmp::max(2 * nf, 1))
                .collect::<Vec<_>>()
                .into_boxed_slice();
            let nb = n - nf;
            let mut ab: Box<[Option<T>]> = iter::repeat_with(Default::default)
                .take(cmp::max(2 * nb, 1))
                .collect::<Vec<_>>()
                .into_boxed_slice();
            for i in 0..nf {
                af[nf - i - 1] = self.get(i);
            }
            for i in 0..nb {
                ab[i] = self.get(nf + i);
            }
            self.front.a = af;
            self.front.n = nf;
            self.back.a = ab;
            self.back.n = nb;
        }
    }
}

impl<T: Clone> List<T> for DualArrayDeque<T> {
    fn size(&self) -> usize {
        self.front.size() + self.back.size()
    }

    fn get(&self, i: usize) -> Option<T> {
        if i < self.front.size() {
            self.front.get(self.front.size() - i - 1)
        } else {
            self.back.get(i - self.front.size())
        }
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        if i < self.front.size() {
            self.front.set(self.front.size() - i - 1, x)
        } else {
            self.back.set(i - self.front.size(), x)
        }
    }

    fn add(&mut self, i: usize, x: T) {
        if i < self.front.size() {
            self.front.add(self.front.size() - i, x)
        } else {
            self.back.add(i - self.front.size(), x)
        }
        self.balance()
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        let x = if i < self.front.size() {
            self.front.remove(self.front.size() - i - i)
        } else {
            self.back.remove(i - self.front.size())
        };
        self.balance();
        x
    }
}

#[cfg(test)]
mod tests {
    use crate::List;

    use super::DualArrayDeque;

    #[test]
    fn test_array_stack() {
        // initialized array has no elements
        let mut deque: DualArrayDeque<i32> = DualArrayDeque::new();
        check_arr_size(&deque, 0, 0, 0);

        // adding elements just increase n
        deque.add(0, 1);
        deque.add(1, 2);
        deque.add(2, 3);
        deque.add(3, 4);
        for i in 0..4 {
            let achieved = deque.get(i);
            let expected = match i {
                0 => Some(1),
                1 => Some(2),
                2 => Some(3),
                3 => Some(4),
                _ => None,
            };
            assert_eq!(achieved, expected)
        }

        // removing
        for i in 0..2 {
            let achieved = deque.get(i);
            let expected = match i {
                0 => Some(1),
                1 => Some(2),
                _ => None,
            };
            assert_eq!(achieved, expected)
        }

        // setting
        let achieved = deque.set(2, 7);
        let expected = Some(3);
        assert_eq!(achieved, expected);
        let achieved = deque.get(2);
        let expected = Some(7);
        assert_eq!(achieved, expected);
    }

    fn check_arr_size<T: Clone>(deque: &DualArrayDeque<T>, l: usize, m: usize, n: usize) {
        assert_eq!(
            (deque.size(), deque.front.size(), deque.back.size()),
            (l, m, n)
        );
    }
}
