#![cfg_attr(not(test), no_std)]

use core::ops::Index;
use trait_set::trait_set;

trait_set! {
    pub trait Entry = Default + Copy + Clone;
}

#[derive(Copy, Clone, Debug)]
pub struct BareMetalQueue<T: Entry, const MAX_STORED: usize> {
    array: [T; MAX_STORED],
    start: usize,
    size: usize,
}

impl<T: Entry, const MAX_STORED: usize> Index<usize> for BareMetalQueue<T, MAX_STORED> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[(self.start + index) % self.array.len()]
    }
}

impl<T: Entry, const MAX_STORED: usize> Default for BareMetalQueue<T, MAX_STORED> {
    fn default() -> Self {
        Self { array: [T::default(); MAX_STORED], start: Default::default(), size: Default::default() }
    }
}

impl <T: Entry, const MAX_STORED: usize> BareMetalQueue<T, MAX_STORED> {
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

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..self.size).map(|i| &self[i])
    }
}

impl<T: Entry, const MAX_STORED: usize> FromIterator<T> for BareMetalQueue<T, MAX_STORED> {
    fn from_iter<V: IntoIterator<Item = T>>(iter: V) -> Self {
        let mut result = Self::new();
        for value in iter {
            result.enqueue(value);
        }
        result
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BareMetalStack<T, const MAX_STORED: usize> {
    array: [T; MAX_STORED],
    top: usize,
}

impl<T: Copy + Clone + Default, const MAX_STORED: usize> Default for BareMetalStack<T, MAX_STORED> {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..self.top).rev().map(|i| &self.array[i])
    }
}


impl<T: Default, const MAX_STORED: usize> Index<usize> for BareMetalStack<T, MAX_STORED> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[self.top - index - 1]
    }
}

impl<T: Entry, const MAX_STORED: usize> FromIterator<T> for BareMetalStack<T, MAX_STORED> {
    fn from_iter<V: IntoIterator<Item = T>>(iter: V) -> Self {
        let mut result = Self::new();
        for value in iter {
            result.push(value);
        }
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

    #[test]
    fn queue_iter_test() {
        let q = (0..10).collect::<BareMetalQueue<usize, 20>>();
        for (i, v) in q.iter().enumerate() {
            assert_eq!(i, *v);
        }
    }

    #[test]
    fn stack_iter_test() {
        let s = (0..10).rev().collect::<BareMetalStack<usize, 20>>();
        for (i, v) in s.iter().enumerate() {
            assert_eq!(i, *v);
        }
    }
}
