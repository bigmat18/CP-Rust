use std::env;
use std::fs;
use std::io;

pub struct SegmentTree {
    tree: Vec<Vec<i32>>,
    max_val: usize,
}

impl SegmentTree {
    pub fn new(c: &[i32]) -> Self {
        let max_val = c.len();

        if max_val == 0 {
            return SegmentTree {
                tree: vec![],
                max_val: 0,
            };
        }

        let mut st = SegmentTree {
            tree: vec![Vec::new(); 4 * max_val],
            max_val,
        };
        st.build(1, 0, max_val - 1, c);
        st
    }

    fn build(&mut self, node: usize, start: usize, end: usize, c: &[i32]) {
        if start == end {
            self.tree[node] = vec![c[start]];
            return;
        }

        let mid = start + (end - start) / 2;
        let left_node = 2 * node;
        let right_node = 2 * node + 1;

        self.build(left_node, start, mid, c);
        self.build(right_node, mid + 1, end, c);

        let mut merged = Vec::with_capacity(end - start + 1);

        let left_vec = &self.tree[left_node];
        let right_vec = &self.tree[right_node];

        let mut left_idx = 0;
        let mut right_idx = 0;

        while left_idx < left_vec.len() && right_idx < right_vec.len() {
            if left_vec[left_idx] <= right_vec[right_idx] {
                merged.push(left_vec[left_idx]);
                left_idx += 1;
            } else {
                merged.push(right_vec[right_idx]);
                right_idx += 1;
            }
        }

        while left_idx < left_vec.len() {
            merged.push(left_vec[left_idx]);
            left_idx += 1;
        }

        while right_idx < right_vec.len() {
            merged.push(right_vec[right_idx]);
            right_idx += 1;
        }

        self.tree[node] = merged;
    }

    pub fn is_there(&self, i: usize, j: usize, k: i32) -> bool {
        return self.is_there_rec(1, 0, self.max_val - 1, i, j, k);
    }

    fn is_there_rec(
        &self,
        node: usize,
        start: usize,
        end: usize,
        i: usize,
        j: usize,
        k: i32,
    ) -> bool {
        if i <= start && j >= end {
            return self.tree[node].binary_search(&k).is_ok();
        }

        let mid = start + (end - start) / 2;
        let left_node = 2 * node;
        let right_node = 2 * node + 1;

        if i <= mid {
            let found_left = self.is_there_rec(left_node, start, mid, i, j, k);
            if found_left {
                return true;
            }
        }

        if j > mid {
            return self.is_there_rec(right_node, mid + 1, end, i, j, k);
        }

        false
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
        "{}/hand-on_02/02_is_there/input/input{}.txt",
        manifest_dir, test_num
    );
    let test_output = format!(
        "{}/hand-on_02/02_is_there/output/output{}.txt",
        manifest_dir, test_num
    );

    let content_in = fs::read_to_string(test_input)?;
    let content_out = fs::read_to_string(test_output)?;

    let mut values_in = content_in.split_whitespace();
    let mut values_out = content_out.split_whitespace();

    let n: usize = values_in.next().unwrap().parse().unwrap();
    let m: usize = values_in.next().unwrap().parse().unwrap();

    let mut segments: Vec<i32> = vec![0; n + 1];
    for _ in 0..n {
        let l: usize = values_in.next().unwrap().parse().unwrap();
        let r: usize = values_in.next().unwrap().parse().unwrap();
        segments[l] += 1;
        segments[r + 1] -= 1;
    }

    let psum: Vec<i32> = segments
        .iter()
        .scan(0, |sum, val| {
            *sum += val;
            Some(*sum)
        })
        .collect();

    let st = SegmentTree::new(&psum);

    for i in 0..m {
        let l: usize = values_in.next().unwrap().parse().unwrap();
        let r: usize = values_in.next().unwrap().parse().unwrap();
        let k: i32 = values_in.next().unwrap().parse().unwrap();

        let result = st.is_there(l, r, k);

        let expected_val: i32 = values_out.next().unwrap().parse().unwrap();
        let expected = expected_val == 1;

        assert_eq!(result, expected, "Error in query index {}", i);
    }

    Ok(())
}
