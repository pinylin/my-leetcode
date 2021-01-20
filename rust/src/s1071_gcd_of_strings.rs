/*
 * @lc app=leetcode.cn id=1071 lang=rust
 *
 * [1071] 字符串的最大公因子
 */

// @lc code=start
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if [str1.clone(), str2.clone()].concat() != [str2.clone(), str1.clone()].concat() {
            "".to_string()
        } else {
            str1[..Self::gcd(str1.len(), str2.len())].to_string()
        }
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}
// @lc code=end

pub struct Solution {}
