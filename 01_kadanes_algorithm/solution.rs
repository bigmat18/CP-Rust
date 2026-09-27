// https://leetcode.com/problems/maximum-subarray/description/

use std::cmp;
use std::io::Read;

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut curr_sum: i32 = nums[0];
        let mut result: i32 = nums[0];

        for num in nums.into_iter().skip(1) {
            if curr_sum < 0 {
                curr_sum = 0;
            }
            curr_sum += num;
            result = cmp::max(curr_sum, result);
        }

        result
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

    let result = Solution::max_sub_array(nums);
    println!("{}", result);
}
