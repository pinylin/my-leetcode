/*
 * @lc app=leetcode id=263 lang=rust
 *
 * [263] Ugly Number
 *
 * https://leetcode.com/problems/ugly-number/description/
 *
 * algorithms
 * Easy (40.45%)
 * Total Accepted:    153.8K
 * Total Submissions: 379.5K
 * Testcase Example:  '6'
 *
 * Write a program to check whether a given number is an ugly number.
 *
 * Ugly numbers are positive numbers whose prime factors only include 2, 3, 5.
 *
 * Example 1:
 *
 *
 * Input: 6
 * Output: true
 * Explanation: 6 = 2 × 3
 *
 * Example 2:
 *
 *
 * Input: 8
 * Output: true
 * Explanation: 8 = 2 × 2 × 2
 *
 *
 * Example 3:
 *
 *
 * Input: 14
 * Output: false
 * Explanation: 14 is not ugly since it includes another prime factor 7.
 *
 *
 * Note:
 *
 *
 * 1 is typically treated as an ugly number.
 * Input is within the 32-bit signed integer range: [−2^31,  2^31 − 1].
 *
 */
impl Solution {
    pub fn is_ugly(mut num: i32) -> bool {
        for &i in [2, 3, 5].iter() {
            while num != 0 && num % i == 0 {
                num /= i;
            }
        }
        num == 1
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_ugly(6), true);
        assert_eq!(Solution::is_ugly(8), true);
        assert_eq!(Solution::is_ugly(14), false);
        assert_eq!(Solution::is_ugly(1), true);
    }
}
