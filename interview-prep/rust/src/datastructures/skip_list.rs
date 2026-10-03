//! Question #84: Skip List
//! Category: Data Structures | Difficulty: Hard | Concepts: geometric distribution, levels
use rand::Rng;

const MAX_LEVEL: usize = 32;

struct SkipNode<K, V> {
    key: K,
    value: V,
    next: Vec<Option<Box<SkipNode<K, V>>>>,
}

pub struct SkipList<K: Ord + Clone, V: Clone> {
    head: Vec<Option<Box<SkipNode<K, V>>>>,
    len: usize,
}

impl<K: Ord + Clone, V: Clone> SkipList<K, V> {
    pub fn new() -> Self {
        Self { head: (0..MAX_LEVEL).map(|_| None).collect(), len: 0 }
    }

    fn random_level(&self) -> usize {
        let mut rng = rand::thread_rng();
        let mut level = 1;
        while rng.gen::<f64>() < 0.5 && level < MAX_LEVEL { level += 1; }
        level
    }

    pub fn insert(&mut self, key: K, value: V) {
        // Check if key exists at level 0
        if self.search(&key).is_some() {
            // Update: find and update the node
            let mut curr = &mut self.head[0];
            while let Some(n) = curr.as_mut() {
                match n.key.cmp(&key) {
                    std::cmp::Ordering::Equal => { n.value = value; return; }
                    std::cmp::Ordering::Less => { curr = &mut n.next[0]; }
                    std::cmp::Ordering::Greater => break,
                }
            }
            return;
        }
        
        // Insert new node at level 0
        self.len += 1;
        let level = self.random_level();
        let mut new_node = Box::new(SkipNode {
            key: key.clone(),
            value,
            next: (0..level).map(|_| None).collect(),
        });
        
        // Find insertion point at level 0
        let mut curr = &mut self.head[0];
        loop {
            match curr {
                None => {
                    *curr = Some(new_node);
                    break;
                }
                Some(n) if n.key.cmp(&key) == std::cmp::Ordering::Greater => {
                    let old = curr.take();
                    new_node.next[0] = old;
                    *curr = Some(new_node);
                    break;
                }
                Some(_) => {
                    curr = &mut curr.as_mut().unwrap().next[0];
                }
            }
        }
    }

    pub fn search(&self, key: &K) -> Option<&V> {
        let mut curr = &self.head[0];
        while let Some(n) = curr {
            match n.key.cmp(key) {
                std::cmp::Ordering::Equal => return Some(&n.value),
                std::cmp::Ordering::Less => curr = &n.next[0],
                std::cmp::Ordering::Greater => break,
            }
        }
        None
    }

    pub fn len(&self) -> usize { self.len }
}

impl<K: Ord + Clone, V: Clone> Default for SkipList<K, V> {
    fn default() -> Self { Self::new() }
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
