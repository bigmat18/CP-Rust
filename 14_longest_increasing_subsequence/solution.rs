// https://www.geeksforgeeks.org/problems/longest-increasing-subsequence-1587115620/1

use std::io::Read;

pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails: Vec<i32> = Vec::new();

        for x in nums {
            let idx = tails.partition_point(|&val| val < x);

            if idx == tails.len() {
                tails.push(x);
            } else {
                tails[idx] = x;
            }
        }

        tails.len() as i32
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

    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        nums.push(values.next().unwrap().parse().unwrap());
    }

    let result = Solution::length_of_lis(nums);
    println!("{}", result);
}
