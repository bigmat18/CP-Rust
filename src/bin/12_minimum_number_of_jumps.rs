// https://www.geeksforgeeks.org/problems/minimum-number-of-jumps-1587115620/1

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut result : i32 = 0;
        let mut idx : i32 = 0;

        while idx < (nums.len()-1) as i32 {
            let mut max = 0;
            let mut best_jump = 0;
            for j in 1..=nums[idx as usize] {
                if (idx + j >= (nums.len()-1) as i32) {
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
