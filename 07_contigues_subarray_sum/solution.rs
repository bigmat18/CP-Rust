// https://leetcode.com/problems/continuous-subarray-sum/description/

use std::collections::HashMap;
use std::io::Read;

pub struct Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false;
        }

        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);

        let mut sum = 0;
        for (i, num) in nums.iter().enumerate() {
            sum += num;

            let key = sum % k;
            if let Some(&j) = map.get(&key) {
                let idx = (i as i32) - (j + 1);
                if idx > 0 {
                    return true;
                }
            } else {
                map.insert(key, i as i32);
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
    let k: i32 = values.next().unwrap().parse().unwrap();

    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        nums.push(values.next().unwrap().parse().unwrap());
    }

    let result = Solution::check_subarray_sum(nums, k);
    println!("{}", result);
}
