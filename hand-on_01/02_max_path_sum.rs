mod binary_tree;
    
use binary_tree::BinaryTree;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_path_sum_leaf_to_leaf() {
        // Albero:
        //        -10 (0)
        //        /     \
        //      9 (1)   20 (2)
        //              /    \
        //            15 (3)  7 (4)
        //
        // Foglie: 9, 15, 7
        // Possibili percorsi foglia-foglia:
        // - 9 -> -10 -> 20 -> 15 = 34
        // - 9 -> -10 -> 20 -> 7  = 26
        // - 15 -> 20 -> 7        = 42 (massimo)
        let mut tree = BinaryTree::with_root(-10); // id 0
        tree.add_node(0, 9, true);           // id 1
        tree.add_node(0, 20, false);         // id 2
        tree.add_node(2, 15, true);          // id 3
        tree.add_node(2, 7, false);          // id 4

        assert_eq!(tree.max_path_sum(), Some(42));
    }

    #[test]
    fn test_max_path_sum_with_deep_branch() {
        // Albero:
        //        1 (0)
        //       / \
        //     2 (1) 3 (2)
        //    /
        //  4 (3)
        //
        // Foglie: 4, 3
        // Unico cammino possibile tra due foglie: 4 -> 2 -> 1 -> 3 = 10
        let mut tree = BinaryTree::with_root(1); // id 0
        tree.add_node(0, 2, true);         // id 1
        tree.add_node(0, 3, false);        // id 2
        tree.add_node(1, 4, true);         // id 3

        assert_eq!(tree.max_path_sum(), Some(10));
    }

    #[test]
    fn test_max_path_sum_minimal_tree() {
        // Albero minimo con due foglie:
        //      5 (0)
        //     / \
        //    3   8
        //
        // Cammino: 3 -> 5 -> 8 = 16
        let mut tree = BinaryTree::with_root(5);
        tree.add_node(0, 3, true);
        tree.add_node(0, 8, false);

        assert_eq!(tree.max_path_sum(), Some(16));
    }

    #[test]
    fn test_max_path_sum_all_negative_values() {
        // Albero con valori tutti negativi:
        //       -5 (0)
        //       /    \
        //    -10 (1) -2 (2)
        //
        // Cammino tra le due foglie: (-10) + (-5) + (-2) = -17
        let mut tree = BinaryTree::with_root(-5);
        tree.add_node(0, -10, true);
        tree.add_node(0, -2, false);

        assert_eq!(tree.max_path_sum(), Some(-17));
    }
}
