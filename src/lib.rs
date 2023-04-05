#![cfg_attr(not(test), no_std)]

#[derive(Clone, Copy, Debug)]
pub struct BareMetalQueue<T, const MAX_STORED: usize> {
    array: [T; MAX_STORED],
    start: usize,
    size: usize,
}

impl <T: Copy + Clone + Default, const MAX_STORED: usize> BareMetalQueue<T, MAX_STORED> {
    pub fn new() -> Self {
        Self {array: [T::default(); MAX_STORED], start: 0, size: 0}
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
            assert_eq!(q.len(), TEST_SIZE - i);
            assert_eq!(q.dequeue(), i);
        }
        assert!(q.is_empty());
    }
}
