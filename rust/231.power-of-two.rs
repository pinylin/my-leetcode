/*
 * @lc app=leetcode id=231 lang=rust
 *
 * [231] Power of Two
 *
 * https://leetcode.com/problems/power-of-two/description/
 *
 * algorithms
 * Easy (41.65%)
 * Total Accepted:    221.4K
 * Total Submissions: 529.5K
 * Testcase Example:  '1'
 *
 * Given an integer, write a function to determine if it is a power of two.
 * 
 * Example 1:
 * 
 * 
 * Input: 1
 * Output: true 
 * Explanation: 2^0 = 1
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 16
 * Output: true
 * Explanation: 2^4 = 16
 * 
 * Example 3:
 * 
 * 
 * Input: 218
 * Output: false
 * 
 */
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n > 0 && (n & n - 1 == 0) {
            return true;
        }
        false
    }
}

