// https://leetcode.com/problems/maximum-subarray/description/

use std::cmp;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut curr_sum : i32 = nums[0];
        let mut result : i32 = nums[0];

        for num in nums.into_iter().skip(1) {
            if curr_sum < 0 {
                curr_sum = 0;
            }
            curr_sum += num;
            result = cmp::max(curr_sum , result);
        }

        result
    }
}