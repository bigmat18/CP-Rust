// https://practice.geeksforgeeks.org/problems/subset-sum-problem/0

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
