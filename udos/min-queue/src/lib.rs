#![forbid(unsafe_code)]

use std::{cmp::min, collections::VecDeque};

#[derive(Default)]
pub struct MinQueue<T> {
    // TODO: your code goes here.
    push_stack: VecDeque<(T, T)>,
    pop_stack: VecDeque<(T, T)>,
    size: usize,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        MinQueue {
            push_stack: VecDeque::new(),
            pop_stack: VecDeque::new(),
            size: 0,
        }
        // TODO: your code goes here.
        // unimplemented!()
    }

    pub fn push(&mut self, val: T) {
        let curr_min = match self.push_stack.is_empty() {
            true => val.clone(),
            false => min(val.clone(), self.push_stack.back().unwrap().1.clone()),
        };
        self.push_stack.push_back((val, curr_min));
        self.size += 1;
        // TODO: your code goes here.
        // unimplemented!()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        if self.pop_stack.is_empty() {
            self.prepare_for_pop();
        }
        let element = self.pop_stack.back().unwrap().0.clone();
        self.pop_stack.pop_back();
        self.size -= 1;
        Some(element)
        // TODO: your code goes here.
        // unimplemented!()
    }

    pub fn front(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }
        if !self.pop_stack.is_empty() {
            return Some(&self.pop_stack.back().unwrap().0);
        }
        return Some(&self.push_stack.front().unwrap().0);
    }

    pub fn min(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }
        if self.pop_stack.is_empty() {
            return Some(&self.push_stack.back().unwrap().1);
        }
        if self.push_stack.is_empty() {
            return Some(&self.pop_stack.back().unwrap().1);
        }
        Some(min(
            &self.push_stack.back().unwrap().1,
            &self.pop_stack.back().unwrap().1,
        ))
        // TODO: your code goes here.
        // unimplemented!()
    }

    pub fn len(&self) -> usize {
        self.size
        // TODO: your code goes here.
        // unimplemented!()
    }

    pub fn is_empty(&self) -> bool {
        let empty = match self.size == 0 {
            true => true,
            false => false,
        };
        empty
        // TODO: your code goes here.
        // unimplemented!()
    }

    fn prepare_for_pop(&mut self) {
        while !self.push_stack.is_empty() {
            let element = self.push_stack.back().unwrap().0.clone();
            self.push_stack.pop_back();
            let curr_min = match self.pop_stack.is_empty() {
                true => element.clone(),
                false => min(element.clone(), self.pop_stack.back().unwrap().1.clone()),
            };
            self.pop_stack.push_back((element, curr_min));
        }
    }
}
