// https://leetcode.com/problems/find-peak-element/description/

use std::cmp::Ordering;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len()-1;

        while (left<right) {
            let center = left + ((right - left) / 2);

            match nums[center].cmp(&nums[center+1]) {
                Ordering::Equal => left = center + 1,
                Ordering::Greater => right = center,
                Ordering::Less => left = center + 1,
            }
        }
        return left as i32;
    }
}