/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 *
 * https://leetcode.com/problems/two-sum/description/
 *
 * algorithms
 * Easy (45.79%)
 * Total Accepted:    3.3M
 * Total Submissions: 7.3M
 * Testcase Example:  '[2,7,11,15]\n9'
 *
 * Given an array of integers nums and an integer target, return indices of the
 * two numbers such that they add up to target.
 * 
 * You may assume that each input would have exactly one solution, and you may
 * not use the same element twice.
 * 
 * You can return the answer in any order.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [2,7,11,15], target = 9
 * Output: [0,1]
 * Output: Because nums[0] + nums[1] == 9, we return [0, 1].
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [3,2,4], target = 6
 * Output: [1,2]
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: nums = [3,3], target = 6
 * Output: [0,1]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 2 <= nums.length <= 10^5
 * -10^9 <= nums[i] <= 10^9
 * -10^9 <= target <= 10^9
 * Only one valid answer exists.
 * 
 * 
 */

use std::collections::HashMap;

#[cfg(test)]
struct Solution {
   
}

impl Solution {

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut store = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let comp = target - num;
            let idx = i as i32;
            if store.contains_key(&comp) {
                return vec![*store.get(&comp).unwrap(), idx];
            }
            store.insert(num, idx);
        }

        return vec![]
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic() {

        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
