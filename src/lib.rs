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
        let result = self.peek();
        self.start = (self.start + 1) % self.array.len();
        self.size -= 1;
        result
    }

    pub fn peek(&self) -> T {
        if self.size == 0 {
            panic!("Queue is empty");
        }
        self.array[self.start]
    }
}

pub struct BareMetalStack<T, const MAX_STORED: usize> {
    array: [T; MAX_STORED],
    top: usize,
}

impl <T: Copy + Clone + Default, const MAX_STORED: usize> BareMetalStack<T, MAX_STORED> {
    pub fn new() -> Self {
        Self {array: [T::default(); MAX_STORED], top: 0}
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn push(&mut self, value: T) {
        if self.top == self.array.len() {
            panic!("Stack is full")
        }
        self.array[self.top] = value;
        self.top += 1;
    }

    pub fn top(&self) -> T {
        if self.top == 0 {
            panic!("Stack is empty")
        }
        self.array[self.top - 1]
    }

    pub fn len(&self) -> usize {
        self.top
    }

    pub fn pop(&mut self) -> T {
        let result = self.top();
        self.top -= 1;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_test1() {
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

    #[test]
    fn queue_test2() {
        let mut q = BareMetalQueue::<usize, 4>::new();
        assert!(q.is_empty());

        for x in 11..15 {
            q.enqueue(x);
            assert!(!q.is_empty());
            assert_eq!(q.len(), x % 10);
            assert_eq!(q.peek(), 11);
        }

        for x in 11..15 {
            let old_len = q.len();
            let v = q.dequeue();
            assert_eq!(x, v);
            assert_eq!(old_len - 1, q.len());
        }

        q.enqueue(12);
        q.enqueue(1);
        assert_eq!(q.dequeue(), 12);
        for x in 2..5 {
            q.enqueue(x);
        }
        for x in 1..5 {
            assert_eq!(x, q.dequeue());
        }
    }

    #[test]
    fn stack_test() {
        let mut stack = BareMetalStack::<usize, 4>::new();
        for x in 11..=14 {
            stack.push(x);
            assert!(!stack.is_empty());
            assert_eq!(stack.len(), x % 10);
            assert_eq!(stack.top(), x);
        }

        for x in (11..=14).rev() {
            let old_len = stack.len();
            let v = stack.pop();
            assert_eq!(x, v);
            assert_eq!(old_len - 1, stack.len());
        }

        stack.push(1);
        stack.push(12);
        assert_eq!(stack.pop(), 12);
        for x in 2..5 {
            stack.push(x);
        }
        for x in (1..5).rev() {
            assert_eq!(x, stack.pop());
        }
    }
}
