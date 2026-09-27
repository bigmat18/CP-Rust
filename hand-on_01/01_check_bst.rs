mod binary_tree;

use binary_tree::BinaryTree;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_bst() {
        let mut tree = BinaryTree::with_root(10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.check_bst(), true);
    }
}
