//! Question #102: LFU Cache
//! Category: Data Structures
//! Difficulty: Hard
//! Concepts: LFU, frequency buckets, ties

use std::collections::{HashMap, VecDeque};
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Entry>>>;

struct Entry {
    key: i32,
    value: i32,
    freq: usize,
}

pub struct LFUCache {
    capacity: usize,
    min_freq: usize,
    cache: HashMap<i32, Rc<RefCell<Entry>>>,
    freqs: HashMap<usize, VecDeque<Rc<RefCell<Entry>>>>,
}

impl LFUCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            min_freq: 0,
            cache: HashMap::new(),
            freqs: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> Option<i32> {
        let entry = self.cache.get(&key)?.clone();
        self.increment(&entry);
        Some(entry.borrow().value)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        if let Some(entry) = self.cache.get(&key) {
            entry.borrow_mut().value = value;
            let entry = entry.clone();
            self.increment(&entry);
            return;
        }
        if self.cache.len() >= self.capacity {
            self.evict();
        }
        let entry = Rc::new(RefCell::new(Entry { key, value, freq: 1 }));
        self.min_freq = 1;
        self.freqs.entry(1).or_default().push_back(entry.clone());
        self.cache.insert(key, entry);
    }

    fn increment(&mut self, entry: &Rc<RefCell<Entry>>) {
        let freq = entry.borrow().freq;
        if let Some(list) = self.freqs.get_mut(&freq) {
            if let Some(pos) = list.iter().position(|e| Rc::ptr_eq(e, entry)) {
                list.remove(pos);
            }
            if freq == self.min_freq && list.is_empty() {
                self.min_freq += 1;
            }
        }
        entry.borrow_mut().freq = freq + 1;
        self.freqs.entry(freq + 1).or_default().push_back(entry.clone());
    }

    fn evict(&mut self) {
        if let Some(list) = self.freqs.get_mut(&self.min_freq) {
            if let Some(entry) = list.pop_front() {
                self.cache.remove(&entry.borrow().key);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut c = LFUCache::new(2);
        c.put(1, 10);
        c.put(2, 20);
        c.get(1);
        c.get(2);
        c.get(1);
        c.put(3, 30);
        assert_eq!(c.get(2), None);
        assert_eq!(c.get(1), Some(10));
    }
}
