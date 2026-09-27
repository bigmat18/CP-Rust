use std::env;
use std::fs;
use std::io;

pub struct SegmentTree {
    size: usize,
    tree: Vec<i32>,
}

impl SegmentTree {
    pub fn new(size: usize) -> Self {
        SegmentTree {
            size,
            tree: vec![-1; 4 * size],
        }
    }

    pub fn update(&mut self, idx: usize, val: i32) {
        self.update_rec(1, 0, self.size-1, idx, val);
    }

    fn update_rec(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i32) {
        if start == end {
            self.tree[node] = val;
            return;
        }

        let mid = start + (end - start) / 2;
        if idx <= mid {
            self.update_rec(2 * node, start, mid, idx, val);
        } else {
            self.update_rec(2 * node + 1, mid + 1, end, idx, val);
        }

        self.tree[node] = self.tree[2 * node].max(self.tree[2 * node + 1]);
    }

    pub fn query_point(&self, target: usize) -> i32 {
        self.query_point_rec(1, 0, self.size-1, target)
    }

    fn query_point_rec(&self, node: usize, start: usize, end: usize, target: usize) -> i32 {
        if start == end {
            return self.tree[node];
        }

        let mid = start + (end - start) / 2;
        if target <= mid {
            self.query_point_rec(2 * node, start, mid, target)
        } else {
            self.query_point_rec(2 * node + 1, mid + 1, end, target)
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
        "{}/src/bin/hand-on_02/02_tests/input{}.txt",
        manifest_dir, test_num
    );
    let test_output = format!(
        "{}/src/bin/hand-on_02/02_tests/output{}.txt",
        manifest_dir, test_num
    );

    let content_in = fs::read_to_string(test_input)?;
    let content_out = fs::read_to_string(test_output)?;

    let mut values_in = content_in.split_whitespace();
    let mut values_out = content_out.split_whitespace();

    let n: usize = values_in.next().unwrap().parse().unwrap();
    let m: usize = values_in.next().unwrap().parse().unwrap();

    let mut diff = vec![0i32; n + 1];
    for _ in 0..n {
        let l: usize = values_in.next().unwrap().parse().unwrap();
        let r: usize = values_in.next().unwrap().parse().unwrap();
        diff[l] += 1;
        diff[r + 1] -= 1;
    }

    let counts: Vec<i32> = diff
        .iter()
        .scan(0, |sum, val| {
            *sum += val;
            Some(*sum)
        })
        .collect();

    let mut queries: Vec<Vec<(usize, usize, usize, i32)>> = vec![Vec::new(); n];
    for idx in 0..m {
        let i: usize = values_in.next().unwrap().parse().unwrap();
        let j: usize = values_in.next().unwrap().parse().unwrap();
        let k: i32 = values_in.next().unwrap().parse().unwrap();

        queries[j].push((idx, i, j, k));
    }

    let mut st = SegmentTree::new(n + 1);
    let mut answers = vec![false; m];

    for pos in 0..n {
        let val = counts[pos] as usize;
        st.update(val, pos as i32);

        for q in &queries[pos] {
            if q.3 < 0 || (q.3 as usize) > n {
                answers[q.0] = false;
            } else {
                let last_seen = st.query_point(q.3 as usize);
                answers[q.0] = last_seen >= (q.1 as i32);
            }
        }
    }

    for i in 0..m {
        let expected_val: i32 = values_out.next().unwrap().parse().unwrap();
        let expected = expected_val == 1;
        assert_eq!(answers[i], expected, "Error in query index {}", i);
    }

    Ok(())
}
