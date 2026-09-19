// https://codeforces.com/contest/609/problem/F?locale=en

use std::io::Read;
use std::collections::BTreeSet;

pub struct SegmentTree {
    tree: Vec<i64>,
    n: usize,
}

impl SegmentTree {
    pub fn new(frogs: &[(i64, i64, usize)]) -> Self {
        let n = frogs.len();
        let tree = vec![0; 4 * n];
        let mut seg_tree = Self { tree, n };
        seg_tree.build(frogs, 1, 0, n - 1);
        seg_tree
    }

    pub fn update(&mut self, idx: usize, new_reach: i64) {
        self.update_rec(1, 0, self.n - 1, idx, new_reach);
    }

    pub fn query(&self, frogs: &[(i64, i64, usize)], p: i64) -> Option<usize> {
        if self.n == 0 {
            return None;
        }
        self.query_rec(frogs, 1, 0, self.n - 1, p)
    }

    fn build(&mut self, frogs: &[(i64, i64, usize)], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = frogs[start].0 + frogs[start].1;
        } else {
            let mid = (start + end) / 2;
            self.build(frogs, Self::left_child(node), start, mid);
            self.build(frogs, Self::right_child(node), mid + 1, end);
            self.tree[node] = self.tree[Self::left_child(node)].max(self.tree[Self::right_child(node)]);
        }
    }

    fn update_rec(&mut self, node: usize, start: usize, end: usize, idx: usize, new_reach: i64) {
        if start == end {
            self.tree[node] = new_reach;
        } else {
            let mid = (start + end) / 2;
            if idx <= mid {
                self.update_rec(Self::left_child(node), start, mid, idx, new_reach);
            } else {
                self.update_rec(Self::right_child(node), mid + 1, end, idx, new_reach);
            }
            self.tree[node] = self.tree[Self::left_child(node)].max(self.tree[Self::right_child(node)]);
        }
    }

    fn query_rec(&self, frogs: &[(i64, i64, usize)], node: usize, start: usize, end: usize, p: i64) -> Option<usize> {
        if frogs[start].0 > p || self.tree[node] < p {
            return None;
        }

        if start == end {
            return Some(start);
        }
        let mid = (start + end) / 2;
        if let Some(res) = self.query_rec(frogs, 2 * node, start, mid, p) {
            return Some(res);
        }

        self.query_rec(frogs, 2 * node + 1, mid + 1, end, p)
    }

    fn left_child(idx: usize) -> usize {
        return 2 * idx;
    }

    fn right_child(idx: usize) -> usize {
        return 2 * idx + 1;
    }
}


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let n : usize = values.next().unwrap().parse().unwrap();
    let m : usize = values.next().unwrap().parse().unwrap();

    let mut frogs : Vec<(i64, i64, usize)> = Vec::with_capacity(n);
    let mut results : Vec<(i64, i64)> = Vec::with_capacity(n);

    for i in 0..n {
        let x : i64 = values.next().unwrap().parse().unwrap();
        let t : i64 = values.next().unwrap().parse().unwrap();
        frogs.push((x, t, i));
        results.push((0, t));
    }

    frogs.sort_by_key(|&(x, _, _)| x);
    let mut segment_tree = SegmentTree::new(&frogs);
    let mut mosquitoes: BTreeSet<(i64, i64, usize)> = BTreeSet::new();

    for _ in 0..m {
        let p : i64 = values.next().unwrap().parse().unwrap();
        let b : i64 = values.next().unwrap().parse().unwrap();

        if let Some(idx) = segment_tree.query(&frogs, p) {
            frogs[idx].1 += b;
            results[frogs[idx].2].0 += 1;
            results[frogs[idx].2].1 += b;

            let frog_x = frogs[idx].0;
            loop {
                let current_reach = frog_x + frogs[idx].1;
                let candidate = mosquitoes
                    .range((frog_x, i64::MIN, 0)..)
                    .next()
                    .cloned();

                if let Some(mos @ (pos, weight, _)) = candidate {
                    if pos <= current_reach {
                        mosquitoes.remove(&mos);
                        frogs[idx].1 += weight;
                        results[frogs[idx].2].0 += 1;
                        results[frogs[idx].2].1 += weight;
                        continue;
                    }
                }
                break;
            }

            segment_tree.update(idx, frog_x + frogs[idx].1);
        } else {
            mosquitoes.insert((p, b, mosquitoes.len()));
        }
    }

    for (killed, total) in results {
        println!("{} {}", killed, total);
    }

}