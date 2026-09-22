use std::cmp;

struct Node {
    key: i32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: i32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }

    fn is_leaf(&self) -> bool {
        self.id_left.is_none() && self.id_right.is_none()
    }
}

pub struct BinaryTree {
    nodes: Vec<Node>,
}

impl BinaryTree {
    pub fn with_root(key: i32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    pub fn add_node(&mut self, parent_id: usize, key: i32, is_left: bool) -> usize {
        assert!(parent_id < self.nodes.len(), "Parent node id does not exist");
        if is_left {
            assert!(
                self.nodes[parent_id].id_left.is_none(),
                "Parent node already has a left child"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
                "Parent node already has a right child"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        if is_left {
            self.nodes[parent_id].id_left = Some(child_id);
        } else {
            self.nodes[parent_id].id_right = Some(child_id);
        }

        child_id
    }

    pub fn check_bst(&self) -> bool {
        if self.nodes.is_empty() {
            return true;
        }
        self.rec_check_bst(0, None, None)
    }

    fn rec_check_bst(&self, node_id: usize, min: Option<i32>, max: Option<i32>) -> bool {
        let node = &self.nodes[node_id];

        if let Some(min_val) = min {
            if node.key <= min_val {
                return false;
            }
        }
        if let Some(max_val) = max {
            if node.key >= max_val {
                return false;
            }
        }

        let left_ok = match node.id_left {
            Some(left_id) => self.rec_check_bst(left_id, min, Some(node.key)),
            None => true,
        };

        let right_ok = match node.id_right {
            Some(right_id) => self.rec_check_bst(right_id, Some(node.key), max),
            None => true,
        };

        left_ok && right_ok
    }

    pub fn max_path_sum(&self) -> Option<i32> {
        if self.nodes.is_empty() {
            return None;
        }

        let mut max_sum = None;
        let root_branch = self.rec_max_leaf_path(0, &mut max_sum);

        max_sum
    }

    fn rec_max_leaf_path(&self, node_id: usize, max_sum: &mut Option<i32>) -> i32 {
        let node = &self.nodes[node_id];

        if node.is_leaf() {
            return node.key;
        }

        let left_sum = node.id_left.map(|id| self.rec_max_leaf_path(id, max_sum));
        let right_sum = node.id_right.map(|id| self.rec_max_leaf_path(id, max_sum));

        match (left_sum, right_sum) {
            (Some(l), Some(r)) => {
                let current_leaf_to_leaf = l + r + node.key;
                *max_sum = Some(match *max_sum {
                    Some(cur) => cmp::max(cur, current_leaf_to_leaf),
                    None => current_leaf_to_leaf,
                });

                node.key + cmp::max(l, r)
            }
            (Some(l), None) => node.key + l,
            (None, Some(r)) => node.key + r,
            (None, None) => unreachable!(),
        }
    }
}
