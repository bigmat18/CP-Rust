// https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/description/

use std::cmp;
use std::io::Read;

pub struct Solution;

impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let max_val = ranges.iter().map(|r| r[1]).max().unwrap_or(0).max(right);
        let mut diff = vec![0; (max_val + 2) as usize];

        for r in ranges {
            let start = r[0] as usize;
            let end = r[1] as usize;
            diff[start] += 1;
            diff[end + 1] -= 1;
        }

        let mut current_coverage = 0;
        for i in 1..=right {
            current_coverage += diff[i as usize];

            if i >= left && current_coverage == 0 {
                return false;
            }
        }

        true
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let n: usize = match values.next() {
        Some(val) => val.parse().unwrap(),
        None => return,
    };

    let mut ranges: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let start: i32 = values.next().unwrap().parse().unwrap();
        let end: i32 = values.next().unwrap().parse().unwrap();
        ranges.push(vec![start, end]);
    }

    let left: i32 = values.next().unwrap().parse().unwrap();
    let right: i32 = values.next().unwrap().parse().unwrap();

    let result = Solution::is_covered(ranges, left, right);
    println!("{}", result);
}
