// https://leetcode.com/problems/find-peak-element/description/

use std::cmp::Ordering;
use std::io::Read;

pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let center = left + ((right - left) / 2);

            match nums[center].cmp(&nums[center + 1]) {
                Ordering::Equal => left = center + 1,
                Ordering::Greater => right = center,
                Ordering::Less => left = center + 1,
            }
        }
        left as i32
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

    let result = Solution::find_peak_element(nums);
    println!("{}", result);
}
