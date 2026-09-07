// https://codeforces.com/problemset/problem/652/D?locale=en

use std::io::{self, Read};

#[derive(Debug)]
pub struct FenwickTree {
    tree: Vec<i64>,
}

impl FenwickTree {
    pub fn with_len(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    pub fn len(&self) -> usize {
        self.tree.len() - 1
    }

    pub fn add(&mut self, i: usize, delta: i64) {
        let mut i = i + 1; 
        assert!(i < self.tree.len());

        while i < self.tree.len() {
            self.tree[i] += delta;
            i = Self::next_sibling(i);
        }
    }

    pub fn sum(&self, i: usize) -> i64 {
        let mut i = i + 1;  

        assert!(i < self.tree.len());
        let mut sum = 0;
        while i != 0 {
            sum += self.tree[i];
            i = Self::parent(i);
        }

        sum
    }

    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.sum(r) - if l == 0 { 0 } else { self.sum(l - 1) }
    }

    fn isolate_trailing_one(i: usize) -> usize {
        if i == 0 {
            0
        } else {
            1 << i.trailing_zeros()
        }
    }

    fn parent(i: usize) -> usize {
        i - Self::isolate_trailing_one(i)
    }

    fn next_sibling(i: usize) -> usize {
        i + Self::isolate_trailing_one(i)
    }
}

#[derive(Clone, Copy, Debug)]
struct Segment {
    original_id: usize,
    l: i64,
    r: i64,
    r_rank: usize,
}

fn main() {
    let mut in_buffer = String::new();
    io::stdin().read_to_string(&mut in_buffer).unwrap();

    let mut values = in_buffer.split_whitespace();
    let n: usize = match values.next() {
        Some(val) => val.parse().unwrap(),
        None => return,
    };

    let mut segments: Vec<Segment> = Vec::with_capacity(n);
    for original_id in 0..n {
        let l: i64 = values.next().unwrap().parse().unwrap();
        let r: i64 = values.next().unwrap().parse().unwrap();
        segments.push(Segment {
            original_id,
            l,
            r,
            r_rank: 0,
        });
    }

    let mut mapping: Vec<usize> = (0..n).collect();
    mapping.sort_unstable_by_key(|&idx| segments[idx].r);

    let mut acc = 0;
    for (i, &idx) in mapping.iter().enumerate() {
        if i > 0 && segments[idx].r != segments[mapping[i - 1]].r {
            acc += 1;
        }
        segments[idx].r_rank = acc;
    }

    segments.sort_unstable_by_key(|seg| seg.l);

    let max_rank = if n > 0 { acc + 1 } else { 0 };
    let mut ft = FenwickTree::with_len(max_rank);

    for seg in &segments {
        ft.add(seg.r_rank, 1);
    }

    let mut results = vec![0; n];
    for seg in &segments {
        let r = seg.r_rank;
        if r > 0 {
            results[seg.original_id] = ft.sum(r - 1);
        } else {
            results[seg.original_id] = 0;
        }
        ft.add(r, -1);
    }

    for res in results {
        println!("{}", res);
    }
}