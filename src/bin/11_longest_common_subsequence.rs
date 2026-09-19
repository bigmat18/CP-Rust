// https://www.geeksforgeeks.org/problems/longest-common-subsequence-1587115620/1

use std::cmp;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let s1 = text1.as_bytes();
        let s2 = text2.as_bytes();
        let n = s1.len();
        let m = s2.len();

        let mut prev = vec![0; n+1];
        let mut curr = vec![0; n+1];

        for i in 1..=m {
            for j in 1..=n {
                if s1[j-1] == s2[i-1] {
                    curr[j] = prev[j-1] + 1;
                } else {
                    curr[j] = cmp::max(prev[j], curr[j-1]);
                }
            }
            prev.copy_from_slice(&curr);
        }
        curr[n] as i32
    }
}
