use crate::List;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub struct ArrayQueue<T>{
    a: Box<[Option]>,
    j : usize, // starting position
    n : usize, // the number of elements in the stack
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
            b[k] = self.a[(self.j + k) % self.a.length()]
        }
        self.a = b;
        self.j = 0;
    }
}

impl<T: Clone> ArrayQueue<T> {
    pub fn add(&mut self, x: T) -> bool {
        if self.n+1 >= self.a.length() {
            self.resize();
        }
        self.a[(self.j+self.n)%self.a.length()] = x;
        self.n += 1;
        true
    }
    pub fn remove(&mut self) -> T {
        let x = self.a[self.j];
        self.j = (self.j+1) % self.a.length();
        self.n -= 1;
        if (self.a.length() >= 3*n) {
            self.resize();
        }
        x
    }
}

#[cfg(test)]
mod tests{
    use super::ArrayQueue;
    use crate::List;


    #[test]
    fn test_array_queue() {
        // initialize
        let q = ArrayQueue<i32>::new(10);
        check_arr_size(&q, 10, 0);
        // add
    }
    fn check_arr_size<T>(queue: &ArrayQueue<T>, length: usize, n: usize) {
        assert_eq!((queue.a.length, queue.n), (length, n));
    }
}