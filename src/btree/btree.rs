use super::node::Node;

pub const TREE_ORDER: usize = 16;

#[derive(Debug)]
pub struct Btree<K, V> {
    root: Node<K, V>,
    size: usize,
}

impl<K, V> Btree<K, V>
where
    K: Copy + PartialEq + PartialOrd + std::fmt::Debug,
    V: Clone + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            root: Node::leaf(),
            size: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            root: Node::leaf_with_cap(capacity),
            size: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn insert(&mut self, key: K, value: V) {
        match self.root.insert(key, value) {
            Ok(()) => {}
            Err((median, left, right)) => {
                println!("Splitting root...");
                let mut new_root = Node::inner();
                new_root.insert_empty_inner(median, vec![left, right]);
                self.root = new_root;
                dbg!(&self);
            }
        }

        self.size += 1;
    }

    fn split(&mut self, keys: &Vec<K>, values: &Vec<V>) {}
}

#[cfg(test)]
mod tests {
    use super::{test_util::assert_inner, *};

    #[test]
    fn btree_splits_root_leaf_node() {
        let mut btree = Btree::with_capacity(3);
        btree.insert("r", 9);
        btree.insert("n", 1);
        btree.insert("a", 8);
        btree.insert("q", 4);

        assert_inner(btree.root, |(keys, nodes)| {
            assert_eq!(keys, &["q"]);
            assert_eq!(nodes.len(), 2);
        })
    }
}

#[cfg(test)]
pub mod test_util {
    use crate::btree::node::Node;

    pub fn assert_leaf<K, V>(node: Node<K, V>, assert_fn: impl Fn((Vec<K>, Vec<V>)) -> ()) {
        match node {
            Node::Leaf { keys, values } => assert_fn((keys, values)),
            _ => assert!(false, "Node is not a Leaf"),
        }
    }

    pub fn assert_inner<K, V>(
        node: Node<K, V>,
        assert_fn: impl Fn((Vec<K>, Vec<Node<K, V>>)) -> (),
    ) {
        match node {
            Node::Inner { keys, nodes } => assert_fn((keys, nodes)),
            _ => assert!(false, "Node is not an Inner"),
        }
    }
}
