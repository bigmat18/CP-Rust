// https://www.geeksforgeeks.org/problems/minimum-number-of-jumps-1587115620/1

use std::io::Read;

pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut result : i32 = 0;
        let mut idx : i32 = 0;

        while idx < (nums.len()-1) as i32 {
            let mut max = 0;
            let mut best_jump = 0;
            for j in 1..=nums[idx as usize] {
                if idx + j >= (nums.len()-1) as i32 {
                    idx += j;
                    break;
                }

                if max <= j + nums[(idx + j) as usize] {
                    best_jump = j;
                    max = j + nums[(idx + j) as usize];
                }
            }
            idx += best_jump;
            result += 1;
        }

        return result;
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
