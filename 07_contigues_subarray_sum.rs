// https://leetcode.com/problems/continuous-subarray-sum/description/

use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false;
        }

        let mut map : HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);

        let mut sum = 0;
        for (i, num) in nums.iter().enumerate() {
            sum += num;

            let key = sum % k;
            if let Some(&j) = map.get(&key) {
                let idx = (i as i32) - (j + 1);
                if (idx > 0) {
                    return true;
                }
            } else {
                map.insert(key, i as i32);
            }
        }

        return false;
    }
}