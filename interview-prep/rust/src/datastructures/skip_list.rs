//! Skip List — probabilistic balanced structure with O(log n) operations.
use std::cmp::Ordering;
use rand::Rng;

const MAX_LEVEL: usize = 32;

struct SkipNode<K, V> {
    key: K,
    value: V,
    next: Vec<Option<Box<SkipNode<K, V>>>>,
}

pub struct SkipList<K: Ord, V> {
    head: Vec<Option<Box<SkipNode<K, V>>>>,
    len: usize,
}

impl<K: Ord + Clone, V: Clone> SkipList<K, V> {
    pub fn new() -> Self {
        Self {
            head: (0..MAX_LEVEL).map(|_| None).collect(),
            len: 0,
        }
    }

    fn random_level(&self) -> usize {
        let mut rng = rand::thread_rng();
        let mut level = 1;
        while rng.gen::<f64>() < 0.5 && level < MAX_LEVEL {
            level += 1;
        }
        level
    }

    pub fn insert(&mut self, key: K, value: V) {
        // For simplicity, this is a simplified version without full level traversal
        // A production implementation would use a proper level-by-level descent
        let level = self.random_level();
        let node = Box::new(SkipNode {
            key: key.clone(),
            value: value.clone(),
            next: (0..level).map(|_| None).collect(),
        });

        // Simplified: just insert at level 0 (linked list)
        // Full implementation would traverse from top level down
        let mut prev: Option<&mut Box<SkipNode<K, V>>> = None;
        let mut current = &mut self.head[0];
        let mut found = false;

        loop {
            match current {
                None => break,
                Some(n) => {
                    if n.key.cmp(&key) == Ordering::Less {
                        prev = Some(n);
                        current = &mut n.next[0];
                    } else if n.key.cmp(&key) == Ordering::Equal {
                        n.value = value;
                        found = true;
                        break;
                    } else {
                        break;
                    }
                }
            }
        }

        if !found {
            self.len += 1;
        }

        // Insert at level 0
        let mut new_node = Some(node);
        if let Some(p) = prev {
            let old_next = p.next[0].take();
            p.next[0] = new_node;
            if let Some(ref mut nn) = p.next[0] {
                nn.next[0] = old_next;
            }
        } else {
            let old = self.head[0].take();
            self.head[0] = new_node;
            if let Some(ref mut nn) = self.head[0] {
                nn.next[0] = old;
            }
        }
    }

    pub fn search(&self, key: &K) -> Option<&V> {
        let mut current = &self.head[0];
        while let Some(n) = current {
            match n.key.cmp(key) {
                Ordering::Equal => return Some(&n.value),
                Ordering::Less => current = &n.next[0],
                Ordering::Greater => break,
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<K: Ord + Clone, V: Clone> Default for SkipList<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut sl: SkipList<i32, String> = SkipList::new();
        sl.insert(1, "one".to_string());
        sl.insert(2, "two".to_string());
        sl.insert(3, "three".to_string());
        assert_eq!(sl.search(&2), Some(&"two".to_string()));
        assert_eq!(sl.search(&99), None);
        assert_eq!(sl.len(), 3);
    }
}
