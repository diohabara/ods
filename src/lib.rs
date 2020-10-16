pub trait Queue<T> {
    fn enqueue(&mut self, other: T);
    fn dequeue(&mut self);
}

pub trait Stack<T> {
    fn enqueue(&mut self, other: T);
    fn dequeue(&mut self);
}

pub trait Deque<T> {
    fn add_first(&mut self, x: T);
    fn remove_first(&mut self);
    fn add_list(&mut self, x: T);
    fn remove_last(&mut self);
}

pub trait List<T> {
    fn size(&self) -> usize;
    fn get(&self, i: usize) -> T;
    fn set(&mut self, i: usize, x: T);
    fn add(&mut self, i: usize, x: T);
    fn remove(&mut self, i: usize);
}

pub trait USet<T> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: T) -> Option<T>;
    fn find(&self, x: T) -> Option<T>;
}

pub trait SSet<T: Ord> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: T) -> Option<T>;
    fn find(&self, x: T) -> Option<T>;
    fn compare(&self, other: T) -> bool;
}