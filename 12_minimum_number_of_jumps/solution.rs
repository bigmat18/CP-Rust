// https://www.geeksforgeeks.org/problems/minimum-number-of-jumps-1587115620/1

use std::io::Read;

pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }

        let mut jumps = 0;
        let mut current_end = 0;
        let mut farthest = 0;

        for i in 0..n - 1 {
            farthest = farthest.max(i + nums[i] as usize);

            if i == current_end {
                jumps += 1;
                current_end = farthest;

                if current_end >= n - 1 {
                    break;
                }
            }
        }

        if current_end < n - 1 {
            return -1;
        }

        jumps
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

    let result = Solution::jump(nums);
    println!("{}", result);
}
