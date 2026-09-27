use std::cmp;
use std::env;
use std::fs;
use std::io;

#[derive(Clone, Copy, Debug)]
pub struct Elem {
    pub val: i32,
    pub to_propagate: bool,
}

pub struct SegmentTree {
    tree: Vec<Elem>,
    max_val: usize,
}

impl SegmentTree {
    pub fn new(max_val: usize) -> Self {
        let default_elem = Elem {
            val: -1,
            to_propagate: false,
        };

        SegmentTree {
            tree: vec![default_elem; 4 * (max_val + 1)],
            max_val,
        }
    }

    pub fn insert(&mut self, idx: usize, val: i32) {
        self.insert_rec(1, 0, self.max_val, idx, val);
    }

    fn insert_rec(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i32) {
        self.tree[node].val = cmp::max(val, self.tree[node].val);

        if start == end {
            return;
        }

        let mid = start + (end - start) / 2;
        let left_node = 2 * node;
        let right_node = 2 * node + 1;

        if idx <= mid {
            self.insert_rec(left_node, start, mid, idx, val);
        } else {
            self.insert_rec(right_node, mid + 1, end, idx, val);
        }
    }

    pub fn update(&mut self, i: usize, j: usize, val: i32) {
        self.update_rec(1, 0, self.max_val, i, j, val);
    }

    fn update_rec(&mut self, node: usize, start: usize, end: usize, i: usize, j: usize, val: i32) -> i32 {

        if i <= start && j >= end {
            self.tree[node].val = cmp::min(val, self.tree[node].val);
            self.tree[node].to_propagate = true;
        } else {

            let mid = start + (end - start) / 2;
            let left_node = 2 * node;
            let right_node = 2 * node + 1;

            if self.tree[node].to_propagate {
                self.tree[node].to_propagate = false;

                self.tree[left_node].val = cmp::min(self.tree[node].val, self.tree[left_node].val);
                self.tree[left_node].to_propagate = true;

                self.tree[right_node].val = cmp::min(self.tree[node].val, self.tree[right_node].val);
                self.tree[right_node].to_propagate = true;
            }

            let left = if i <= mid {
                self.update_rec(left_node, start, mid, i, j, val)
            } else {
                self.tree[left_node].val
            };
            let right = if j > mid {
                self.update_rec(right_node, mid + 1, end, i, j, val)
            } else {
                self.tree[right_node].val
            };

            self.tree[node].val = cmp::max(left, right);
        }

        return self.tree[node].val;
    }

    pub fn max(&mut self, i: usize, j: usize) -> i32 {
        return self.max_rec(1, 0, self.max_val, i, j);
    }

    fn max_rec(&mut self, node: usize, start: usize, end: usize, i: usize, j: usize) -> i32 {
        if i <= start && j >= end {
            return self.tree[node].val;
        } else {
            let mid = start + (end - start) / 2;
            let left_node = 2 * node;
            let right_node = 2 * node + 1;

            if self.tree[node].to_propagate {
                self.tree[node].to_propagate = false;

                self.tree[left_node].val = cmp::min(self.tree[node].val, self.tree[left_node].val);
                self.tree[left_node].to_propagate = true;

                self.tree[right_node].val = cmp::min(self.tree[node].val, self.tree[right_node].val);
                self.tree[right_node].to_propagate = true;
            }

            let left = if i <= mid { self.max_rec(left_node, start, mid, i, j) } else { -1 };
            let right = if j > mid { self.max_rec(right_node, mid + 1, end, i, j) } else { -1 };
            
            return cmp::max(left, right);
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        std::process::exit(1);
    }

    let test_num = &args[1];
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let test_input = format!(
        "{}/hand-on_02/01_min_max/input/input{}.txt",
        manifest_dir, test_num
    );
    let test_output = format!(
        "{}/hand-on_02/01_min_max/output/output{}.txt",
        manifest_dir, test_num
    );

    let content_in = fs::read_to_string(test_input)?;
    let content_out = fs::read_to_string(test_output)?;

    let mut values_in = content_in.split_whitespace();
    let mut values_out = content_out.split_whitespace();

    let n : usize = values_in.next().unwrap().parse().unwrap();
    let m : usize = values_in.next().unwrap().parse().unwrap();

    let mut seg_tree = SegmentTree::new(n);
    for i in 1..=n {
        let val = values_in.next().unwrap().parse().unwrap();
        seg_tree.insert(i, val);
    }

    for k in 0..m {
        let query_type : usize = values_in.next().unwrap().parse().unwrap();
        let i : usize = values_in.next().unwrap().parse().unwrap();
        let j : usize = values_in.next().unwrap().parse().unwrap();
        if query_type == 0 {
            let val : i32 = values_in.next().unwrap().parse().unwrap();
            seg_tree.update(i, j, val);
        } else if query_type == 1 {
            let expected: i32 = values_out.next().unwrap().parse().unwrap();
            let res = seg_tree.max(i, j);
            assert_eq!(res, expected, "Error in index {}", k);
        } else {
            panic!("Invalid query type {}", query_type);
        }
    }

    Ok(())
}
