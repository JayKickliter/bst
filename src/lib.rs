#![cfg_attr(debug_assertions, allow(dead_code))]

use std::{cmp::Ordering, default::Default, mem};

#[derive(Debug, PartialEq, Clone)]
pub struct Tree<K, V>(Option<Box<Node<K, V>>>);

impl<K, V> Default for Tree<K, V> {
    fn default() -> Self {
        Self(None)
    }
}

impl<K: Ord, V> Tree<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(k: K, v: V) -> Self {
        let mut tree = Self::new();
        tree.insert(k, v);
        tree
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        match &mut self.0 {
            inner @ None => {
                let _ = mem::replace(inner, Some(Box::new(Node::new(k, v))));
                None
            }
            Some(node) => node.as_mut().insert(k, v),
        }
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.0.as_ref().and_then(|node| node.get(k))
    }

    pub fn len(&self) -> usize {
        self.0.as_ref().map_or(0, |node| node.len())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn iter(&self) -> TreeIter<K, V> {
        TreeIter::new(self)
    }
}

/// A node in a binary search tree
#[derive(Debug, PartialEq, Clone)]
pub struct Node<K, V> {
    /// This node's k
    k: K,
    /// This node's v
    v: V,
    /// L child
    l: Option<Box<Self>>,
    /// R child
    r: Option<Box<Self>>,
}

impl<K: Ord, V> Node<K, V> {
    pub(crate) fn new(k: K, v: V) -> Self {
        Self {
            k,
            v,
            l: None,
            r: None,
        }
    }

    pub(crate) fn insert(&mut self, k: K, v: V) -> Option<V> {
        let lr = match self.k.cmp(&k) {
            Ordering::Greater => &mut self.l,
            Ordering::Equal => {
                return Some(mem::replace(&mut self.v, v));
            }
            Ordering::Less => &mut self.r,
        };
        match lr {
            None => {
                *lr = Some(Box::new(Self::new(k, v)));
                None
            }
            Some(node) => node.as_mut().insert(k, v),
        }
    }

    pub(crate) fn get(&self, k: &K) -> Option<&V> {
        let lr = match self.k.cmp(k) {
            Ordering::Greater => &self.l,
            Ordering::Equal => return Some(&self.v),
            Ordering::Less => &self.r,
        };
        match lr {
            None => None,
            Some(node) => node.as_ref().get(k),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.l.as_ref().map_or(0, |node| node.len())
            + 1
            + self.r.as_ref().map_or(0, |node| node.len())
    }
}

pub struct TreeIter<'a, K, V> {
    curr: Option<&'a Node<K, V>>,
    stack: Vec<&'a Node<K, V>>,
}

impl<'a, K, V> TreeIter<'a, K, V> {
    pub fn new(tree: &'a Tree<K, V>) -> Self {
        Self {
            curr: tree.0.as_deref(),
            stack: Vec::new(),
        }
    }
}

impl<'a, K, V> Iterator for TreeIter<'a, K, V> {
    type Item = &'a Node<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(curr) = self.curr {
            self.stack.push(curr);
            self.curr = curr.l.as_deref();
        }
        if let Some(it) = self.stack.pop() {
            self.curr = it.r.as_deref();
            Some(it)
        } else {
            None
        }
    }
}

pub(crate) fn l<K, V>(root: &Option<Box<Node<K, V>>>) -> Option<&Node<K, V>> {
    match root {
        None => None,
        Some(box_root) => box_root.l.as_ref().map(|box_node| box_node.as_ref()),
    }
}
pub(crate) fn r<K, V>(root: &Option<Box<Node<K, V>>>) -> Option<&Node<K, V>> {
    match root {
        None => None,
        Some(box_root) => box_root.r.as_ref().map(|box_node| box_node.as_ref()),
    }
}
pub(crate) fn l_mut<K, V>(root: &mut Option<Box<Node<K, V>>>) -> Option<&mut Node<K, V>> {
    match root {
        None => None,
        Some(box_root) => box_root.l.as_mut().map(|box_node| box_node.as_mut()),
    }
}
pub(crate) fn len<K: Ord, V>(root: &Option<Box<Node<K, V>>>) -> Option<usize> {
    root.as_ref().map(|node| node.len())
}
pub(crate) fn r_mut<K, V>(root: &mut Option<Box<Node<K, V>>>) -> Option<&mut Node<K, V>> {
    match root {
        None => None,
        Some(box_root) => box_root.r.as_mut().map(|box_node| box_node.as_mut()),
    }
}
pub(crate) fn rotate_r<K, V>(root: &mut Option<Box<Node<K, V>>>) {
    let new_root = if let Some(mut new_r) = root.take() {
        if let Some(mut new_root) = new_r.l.take() {
            new_root.r = Some(new_r);
            new_root
        } else {
            new_r
        }
    } else {
        return;
    };
    *root = Some(new_root);
}

fn rotate_l<K, V>(root: &mut Option<Box<Node<K, V>>>) {
    let new_root = if let Some(mut new_l) = root.take() {
        if let Some(mut new_root) = new_l.r.take() {
            new_root.l = Some(new_l);
            new_root
        } else {
            new_l
        }
    } else {
        return;
    };
    *root = Some(new_root);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_eq_pass() {
        let tree_a = Tree::with(String::from("cat"), String::from("meow"));
        let tree_b = Tree::with(String::from("cat"), String::from("meow"));
        assert_eq!(tree_a, tree_b);
    }

    #[test]
    #[should_panic]
    fn tree_eq_fail() {
        let tree_a = Tree::with(String::from("cat"), String::from("meow"));
        let tree_b = Tree::with(String::from("dog"), String::from("bark"));
        assert_eq!(tree_a, tree_b);
    }

    #[test]
    fn tree_neq_pass() {
        let tree_a = Tree::with(String::from("cat"), String::from("meow"));
        let tree_b = Tree::with(String::from("dog"), String::from("bark"));
        assert_ne!(tree_a, tree_b);
    }

    #[test]
    #[should_panic]
    fn tree_neq_fail() {
        let tree_a = Tree::with(String::from("cat"), String::from("meow"));
        let tree_b = Tree::with(String::from("cat"), String::from("meow"));
        assert_ne!(tree_a, tree_b);
    }

    #[test]
    fn tree_insert_pass() {
        let mut tree_root = Tree::with(1, '1');
        tree_root.insert(0, '0');
        tree_root.insert(2, '2');
        let tree_root_1 = Tree(Some(Box::new(Node {
            k: 1,
            v: '1',
            l: Some(Box::new(Node::new(0, '0'))),
            r: Some(Box::new(Node::new(2, '2'))),
        })));
        assert_eq!(tree_root, tree_root_1);
        assert_eq!(tree_root.len(), 3);
    }

    #[test]
    fn tree_insert_duplicate_pass() {
        let mut tree_root = Tree::with(0, '0');
        assert_eq!(tree_root.insert(1, '1'), None);
        assert_eq!(tree_root.insert(1, '1'), Some('1'));
    }

    #[test]
    fn tree_test_get_pass() {
        let mut tree_root = Tree::with(1, '1');
        tree_root.insert(0, '0');
        tree_root.insert(2, '2');
        assert_eq!(tree_root.get(&0), Some(&'0'));
        assert_eq!(tree_root.get(&1), Some(&'1'));
        assert_eq!(tree_root.get(&2), Some(&'2'));
    }

    #[test]
    fn tree_test_iter_pass() {
        let mut tree: Tree<u8, ()> = Tree::new();
        for _ in 0..100 {
            tree.insert(rand::random(), ());
        }
        let mut iter = tree.iter();
        let mut last = iter.next().unwrap().k;
        for &Node { k, .. } in iter {
            assert!(k > last);
            last = k;
        }
    }

    #[test]
    fn node_rotate_r_pass() {
        let mut node_root = Node::new(1, '1');
        node_root.insert(0, '0');
        node_root.insert(2, '2');
        assert_eq!(node_root.l.as_ref().map_or(0, |node| node.len()), 1);
        assert_eq!(node_root.r.as_ref().map_or(0, |node| node.len()), 1);
        let mut node_root = Some(Box::new(node_root));
        rotate_r(&mut node_root);
        assert_eq!(l(&node_root).map(|node| node.len()), None);
        assert_eq!(r(&node_root).map(|node| node.len()), Some(2));
    }

    #[test]
    fn node_rotate_l_pass() {
        let mut node_root = Node::new(1, '1');
        node_root.insert(0, '0');
        node_root.insert(2, '2');
        assert_eq!(node_root.r.as_ref().map_or(0, |node| node.len()), 1);
        assert_eq!(node_root.l.as_ref().map_or(0, |node| node.len()), 1);
        let mut node_root = Some(Box::new(node_root));
        rotate_l(&mut node_root);
        assert_eq!(r(&node_root).map(|node| node.len()), None);
        assert_eq!(l(&node_root).map(|node| node.len()), Some(2));
    }
}
