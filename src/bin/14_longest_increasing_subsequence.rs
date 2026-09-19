// https://www.geeksforgeeks.org/problems/longest-increasing-subsequence-1587115620/1

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails: Vec<i32> = Vec::new();

        for x in nums {
            let idx = tails.partition_point(|&val| val < x);

            if idx == tails.len() {
                tails.push(x);
            } else {
                tails[idx] = x;
            }
        }

        tails.len() as i32
    }
}
