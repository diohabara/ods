use crate::List;
use std::cmp;
use std::iter;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Array<T> {
    a: Box<[Option<T>]>,
    length: usize, // heaped size
    n: usize,      // size of this array
}

impl<T> Array<T> {
    fn alloc_boxed_slice(size: usize) -> Box<[Option<T>]> {
        iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
    pub fn new(len: usize) -> Self {
        Self {
            a: Self::alloc_boxed_slice(len),
            length: len,
            n: 0,
        }
    }
    pub fn resize(&mut self) {
        let mut b = Self::alloc_boxed_slice(cmp::max(self.n * 2, 1));
        for i in 0..self.n {
            b[i] = self.a[i].take();
        }
        self.a = b;
        self.length = self.n*2;
    }
    pub fn print_array(&self) {
        println!("length: {}, n: {}", &self.length, &self.n);
    }
}

impl<T: Clone> List<T> for Array<T> {
    fn size(&self) -> usize {
        self.n
    }
    fn get(&self, i: usize) -> Option<T> {
        self.a[i].clone()
    }
    fn set(&mut self, i: usize, x: T) -> Option<T> {
        let y = self.a[i].take();
        self.a[i] = Some(x);
        y
    }
    fn add(&mut self, i: usize, x: T) {
        if self.n + 1 >= self.length {
            self.resize();
        }
        for j in (i + 1..=self.n).rev() {
            self.a[j] = self.a[j - 1].take();
        }
        self.a[i] = Some(x);
        self.n += 1;
    }
    fn remove(&mut self, i: usize) -> Option<T> {
        let x = self.a[i].take();
        for j in i..self.n - 1 {
            self.a[j] = self.a[j + 1].take();
        }
        self.n -= 1;
        if self.length >= 3 * self.n {
            self.resize();
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::Array;
    use crate::List;

    #[test]
    fn test_array() {
        // initialized array has no elements
        let mut array: Array<i32> = Array::new(10);
        check_arr_size(&array, 10, 0);

        // adding elements just increase n
        array.add(0, -1);
        array.add(1, 1);
        for (i, elem) in [-1, 1].iter().enumerate() {
            assert_eq!(array.get(i), Some(*elem));
        }
        check_arr_size(&array, 10, 2);

        // removing elements just decrease n
        let achieved = array.remove(0);
        let expected = Some(-1);
        assert_eq!(achieved, expected);
        array.print_array();
        check_arr_size(&array, 2, 1);

        // setting elements
        for i in 1..9 {
            array.add(i, 0);
        }
        check_arr_size(&array, 10, 9);
        for (i, elem) in [2, 3, 4, 5, 6, 7, 8, 9].iter().enumerate() {
            let achieved = array.set(i + 1, *elem);
            let expected = Some(0);
            assert_eq!(achieved, expected);
        }
        for (i, elem) in [1, 2, 3, 4, 5, 6, 7, 8, 9].iter().enumerate() {
            assert_eq!(array.get(i), Some(*elem));
        }
    }

    fn check_arr_size<T>(array: &Array<T>, length: usize, n: usize) {
        assert_eq!((array.length, array.n), (length, n));
    }
}
