//! Question #96: Red-Black Tree
//! Category: Data Structures | Difficulty: Hard | Concepts: red-black tree, invariants, rotations

use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq)]
enum Color { Red, Black }

struct Node<K, V> {
    key: K,
    value: V,
    color: Color,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

pub struct RBTree<K: Ord, V> {
    root: Option<Box<Node<K, V>>>,
    size: usize,
}

impl<K: Ord, V> RBTree<K, V> {
    pub fn new() -> Self {
        Self { root: None, size: 0 }
    }

    pub fn insert(&mut self, key: K, value: V) {
        Self::insert_inner(&mut self.root, key, value);
        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }
        self.size += 1;
    }

    fn insert_inner(node: &mut Option<Box<Node<K, V>>>, key: K, value: V) {
        match node {
            None => {
                *node = Some(Box::new(Node { key, value, color: Color::Red, left: None, right: None }));
            }
            Some(n) => {
                match key.cmp(&n.key) {
                    Ordering::Less => Self::insert_inner(&mut n.left, key, value),
                    Ordering::Greater => Self::insert_inner(&mut n.right, key, value),
                    Ordering::Equal => { n.value = value; }
                }
            }
        }
    }

    pub fn search<'a>(&'a self, key: &'a K) -> Option<&'a V> {
        Self::search_inner(&self.root, key)
    }

    fn search_inner<'a>(node: &'a Option<Box<Node<K, V>>>, key: &'a K) -> Option<&'a V> {
        match node {
            None => None,
            Some(n) => {
                match key.cmp(&n.key) {
                    Ordering::Less => Self::search_inner(&n.left, key),
                    Ordering::Greater => Self::search_inner(&n.right, key),
                    Ordering::Equal => Some(&n.value),
                }
            }
        }
    }

    pub fn len(&self) -> usize { self.size }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut t: RBTree<i32, String> = RBTree::new();
        t.insert(1, "one".to_string());
        t.insert(2, "two".to_string());
        t.insert(3, "three".to_string());
        assert_eq!(t.search(&2), Some(&"two".to_string()));
        assert_eq!(t.search(&99), None);
        assert_eq!(t.len(), 3);
    }
}
