#![cfg_attr(not(test), no_std)]

use core::ops::Index;

#[derive(Copy, Clone, Debug)]
pub struct BareMetalQueue<T: Default, const MAX_STORED: usize> {
    array: [T; MAX_STORED],
    start: usize,
    size: usize,
}

impl<T: Copy + Clone + Default, const MAX_STORED: usize> Default for BareMetalQueue<T, MAX_STORED> {
    fn default() -> Self {
        Self { array: [T::default(); MAX_STORED], start: Default::default(), size: Default::default() }
    }
}

impl <T: Copy + Clone + Default, const MAX_STORED: usize> BareMetalQueue<T, MAX_STORED> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn enqueue(&mut self, value: T) {
        if self.size == self.array.len() {
            panic!("Queue is full");
        }
        let index = (self.start + self.size) % self.array.len();
        self.array[index] = value;
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> T {
        if self.size == 0 {
            panic!("Queue is empty");
        }
        let result = self.array[self.start];
        self.start = (self.start + 1) % self.array.len();
        self.size -= 1;
        result
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.array.iter()
    }
}

impl<T: Default, const MAX_STORED: usize> Index<usize> for BareMetalQueue<T, MAX_STORED> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[(self.start + index) % self.array.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        const TEST_SIZE: usize = 10;

        let mut q: BareMetalQueue<usize, TEST_SIZE> = BareMetalQueue::new();
        assert!(q.is_empty());
        for i in 0..TEST_SIZE {
            q.enqueue(i);
            assert_eq!(q.len(), i + 1);
        }
        assert!(!q.is_empty());

        for i in 0..TEST_SIZE {
            assert_eq!(i, q[i]);
        }

        for i in 0..TEST_SIZE {
            assert_eq!(q.len(), TEST_SIZE - i);
            assert_eq!(q.dequeue(), i);
        }
        assert!(q.is_empty());

        for i in 0..TEST_SIZE {
            q.enqueue(i);
        }
        for i in 0..TEST_SIZE / 2 {
            q.dequeue();
            q.enqueue(i + TEST_SIZE);
        }
        for i in 0..q.len() {
            assert_eq!(i + TEST_SIZE / 2, q[i]);
        }
    }
}
