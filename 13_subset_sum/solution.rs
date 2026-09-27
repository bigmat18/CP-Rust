// https://practice.geeksforgeeks.org/problems/subset-sum-problem/0

use std::io::Read;

pub struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum_val: i32 = nums.iter().sum();
        if sum_val % 2 != 0 {
            return false;
        }

        let target = (sum_val / 2) as usize;

        let mut dp = vec![false; target + 1];
        dp[0] = true;

        for num in nums {
            let num = num as usize;
            if num > target {
                return false;
            }

            for j in (num..=target).rev() {
                if dp[j - num] {
                    dp[j] = true;
                }
            }

            if dp[target] {
                return true;
            }
        }

        dp[target]
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

    let result = Solution::can_partition(nums);
    println!("{}", result);
}
