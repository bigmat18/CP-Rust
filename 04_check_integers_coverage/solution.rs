// https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/description/

use std::cmp;
use std::io::Read;

pub struct Solution;

impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut target = right - left + 1;
        let mut map: Vec<bool> = vec![false; target as usize];

        for range in &ranges {
            if range[0] < left && range[1] < left {
                continue;
            } 

            if range[0] > right && range[1] > right {
                continue;
            }

            for num in cmp::max(range[0], left)..=cmp::min(range[1], right) {
                let i : usize = (num - left) as usize;
                if !map[i] {
                    map[i] = true;
                    target -= 1;
                }

                if target == 0 {
                    return true;
                }
            }
        }

        return false;
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
