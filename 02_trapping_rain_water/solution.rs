// https://leetcode.com/problems/trapping-rain-water/description/

use std::cmp;
use std::io::Read;

pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        } 
        let mut result = 0;
        let mut left = 1;
        let mut right = height.len() - 2;

        let mut l_max = height[0];
        let mut r_max = height[height.len() - 1];

        while left <= right {
            if l_max <= r_max {
                result += cmp::max(0, l_max - height[left]);
                l_max = cmp::max(l_max, height[left]);
                left += 1;
            } else {
                result += cmp::max(0, r_max - height[right]);
                r_max = cmp::max(r_max, height[right]);
                right -= 1;
            }
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

    let mut height: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        height.push(values.next().unwrap().parse().unwrap());
    }

    let result = Solution::trap(height);
    println!("{}", result);
}
