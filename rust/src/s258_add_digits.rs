/*
 * @lc app=leetcode id=258 lang=rust
 *
 * [258] Add Digits
 *
 * https://leetcode.com/problems/add-digits/description/
 *
 * algorithms
 * Easy (53.73%)
 * Total Accepted:    234.4K
 * Total Submissions: 435K
 * Testcase Example:  '38'
 *
 * Given a non-negative integer num, repeatedly add all its digits until the
 * result has only one digit.
 * 
 * Example:
 * 
 * 
 * Input: 38
 * Output: 2 
 * Explanation: The process is like: 3 + 8 = 11, 1 + 1 = 2. 
 * Since 2 has only one digit, return it.
 * 
 * 
 * Follow up:
 * Could you do it without any loop/recursion in O(1) runtime?
 */
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let ret = num.to_string().bytes().fold(0, |acc, c| acc + (c - b'0') as i32);
        if ret >= 10 {
            Self::add_digits(ret)
        } else {
            ret
        }
    }

    pub fn add_digits_2(num: i32) -> i32 {
        1 + (num - 1) % 9
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::add_digits(38), 2);
    }
}