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
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left == None,
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right == None,
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    pub fn check_bst(&self) -> bool {
        self.rec_check_bst(0)
    }

    fn rec_check_bst(&self, node_id: usize) -> bool {
        let node = &self.nodes[node_id];

        if let Some(left_id) = node.id_left {
            if self.nodes[left_id].key >= node.key {
                return false;
            }
        }

        if let Some(right_id) = node.id_right {
            if self.nodes[right_id].key <= node.key {
                return false;
            }
        }

        let left_bst = if let Some(left_id) = node.id_left {
            self.rec_check_bst(left_id)
        } else {
            true
        };

        let right_bst = if let Some(right_id) = node.id_right {
            self.rec_check_bst(right_id)
        } else {
            true
        };

        left_bst && right_bst
    }

    pub fn max_path_sum(&self) -> i32 {
        if self.nodes.is_empty() {
            return 0;
        }

        let mut result = i32::MIN;
        let val = self.max_path_sum_rec(0, &mut result);
        if result == i32::MIN {
            return val;
        } else {
            return result;
        }
    }

    fn max_path_sum_rec(&self, node_id: usize, max_sum: &mut i32) -> i32 {
        let node = &self.nodes[node_id];

        let left_val = node.id_left.map(|id| self.max_path_sum_rec(id, max_sum));
        let right_val = node.id_right.map(|id| self.max_path_sum_rec(id, max_sum));

        match (left_val, right_val) {
            (Some(l_val), Some(r_val)) => {
                *max_sum = cmp::max(*max_sum, l_val + r_val + node.key);
                node.key + cmp::max(l_val, r_val)
            }
            (Some(l_val), None) => node.key + l_val,
            (None, Some(r_val)) => node.key + r_val,
            (None, None) => node.key,
        }
    }
}

