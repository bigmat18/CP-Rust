// https://leetcode.com/problems/trapping-rain-water/description/

use std::cmp;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if (height.len() < 3) {
            return 0;
        } 
        let mut result = 0;
        let mut left = 1;
        let mut right = height.len() - 2;

        let mut lMax = height[0];
        let mut rMax = height[height.len() - 1];

        while (left <= right) {
            if (lMax <= rMax) {
                result += cmp::max(0, lMax - height[left]);
                lMax = cmp::max(lMax, height[left]);
                left += 1;
            } else {
                result += cmp::max(0, rMax - height[right]);
                rMax = cmp::max(rMax, height[right]);
                right -= 1;
            }
        }

        return result;
    }
}