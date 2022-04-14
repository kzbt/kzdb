use super::{btree::TREE_ORDER, Searchable};

const NODE_CAPACITY: usize = TREE_ORDER - 1;

pub struct Kv<K, V> {
    pub key: K,
    pub value: V,
}

impl<K, V> Kv<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

#[derive(Debug)]
pub enum Node<K, V> {
    Inner {
        keys: Vec<K>,
        nodes: Vec<Node<K, V>>,
    },
    Leaf {
        keys: Vec<K>,
        values: Vec<V>,
    },
}

type SplitNodes<K, V> = (K, Node<K, V>, Node<K, V>);

impl<K, V> Node<K, V>
where
    K: Copy + PartialOrd + PartialEq + std::fmt::Debug,
    V: Clone,
{
    pub fn inner() -> Self {
        Self::Inner {
            keys: Vec::with_capacity(NODE_CAPACITY),
            nodes: Vec::with_capacity(TREE_ORDER),
        }
    }

    pub fn leaf() -> Self {
        Self::Leaf {
            keys: Vec::with_capacity(NODE_CAPACITY),
            values: Vec::with_capacity(NODE_CAPACITY),
        }
    }

    pub fn leaf_with_cap(capacity: usize) -> Self {
        Self::Leaf {
            keys: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<(), SplitNodes<K, V>> {
        match self {
            Node::Leaf { keys, values } => {
                let idx = keys.search(key).unwrap_or_else(|pos| pos);
                if keys.len() < (dbg!(keys.capacity())) {
                    keys.insert(idx, key);
                    values.insert(idx, value);
                    Ok(())
                } else {
                    // split self
                    let (median, left, right) = self.split(idx, key, value);
                    Err((median, left, right))
                }
            }
            Node::Inner { keys, nodes } => {
                let idx = keys.search(key).unwrap_or_else(|pos| pos);
                let node = &mut nodes[idx];
                match node.insert(key, value) {
                    Ok(()) => Ok(()),
                    Err((median, left, right)) => {
                        keys.insert(idx, median);
                        nodes.insert(idx, left);
                        nodes.insert(idx + 1, right);
                        Ok(())
                    }
                }
            }
        }
    }

    pub fn insert_empty_inner(&mut self, key: K, mut ns: Vec<Node<K, V>>) {
        if let Node::Inner { keys, nodes } = self {
            keys.push(key);
            nodes.append(&mut ns);
        }
    }

    fn split(&mut self, idx: usize, key: K, value: V) -> SplitNodes<K, V> {
        match self {
            Node::Leaf { keys, values } => {
                keys.insert(idx, key);
                values.insert(idx, value);
                let mid = keys.len() / 2;
                let median = keys[mid];
                let left_keys = keys;
                let right_keys = left_keys.drain(mid..).collect();

                dbg!(&left_keys);
                dbg!(&right_keys);

                let left_values = values;
                let right_values = left_values.drain(mid..).collect();
                (
                    median,
                    Node::Leaf {
                        keys: left_keys.drain(..).collect(),
                        values: left_values.drain(..).collect(),
                    },
                    Node::Leaf {
                        keys: right_keys,
                        values: right_values,
                    },
                )
            }
            Node::Inner { keys, nodes } => (key, Node::leaf(), Node::leaf()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::btree::btree::test_util::assert_leaf;

    #[test]
    fn leaf_inserts_in_order() {
        let mut node = Node::leaf_with_cap(3);
        let _ = node.insert("b", 9);
        let _ = node.insert("d", 4);
        let _ = node.insert("a", 8);

        assert_leaf(node, |(keys, values)| {
            assert_eq!(keys, &["a", "b", "d"]);
            assert_eq!(values, &[8, 9, 4]);
        })
    }
}
