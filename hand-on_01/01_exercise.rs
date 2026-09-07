struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_bst() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.check_bst(), true);
    }
}