/*
 * @lc app=leetcode id=66 lang=rust
 *
 * [66] Plus One
 *
 * https://leetcode.com/problems/plus-one/description/
 *
 * algorithms
 * Easy (40.71%)
 * Total Accepted:    360.3K
 * Total Submissions: 883.8K
 * Testcase Example:  '[1,2,3]'
 *
 * Given a non-empty array of digits representing a non-negative integer, plus
 * one to the integer.
 * 
 * The digits are stored such that the most significant digit is at the head of
 * the list, and each element in the array contain a single digit.
 * 
 * You may assume the integer does not contain any leading zero, except the
 * number 0 itself.
 * 
 * Example 1:
 * 
 * 
 * Input: [1,2,3]
 * Output: [1,2,4]
 * Explanation: The array represents the integer 123.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [4,3,2,1]
 * Output: [4,3,2,2]
 * Explanation: The array represents the integer 4321.
 * 
 */
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut c = 0;
        for i in (0..digits.len()).rev() {
            digits[i] = if digits[i] == 9 {
                c = 1;
                0
            } else {
                c = 0;
                digits[i] + 1
            };
            if c == 0 { break }
        }
        if c == 1 {digits.insert(0, 1)}
        digits
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![1, 2, 9]), vec![1, 3, 0]);
        assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }
}