/*
 * @lc app=leetcode id=168 lang=rust
 *
 * [168] Excel Sheet Column Title
 *
 * https://leetcode.com/problems/excel-sheet-column-title/description/
 *
 * algorithms
 * Easy (28.67%)
 * Total Accepted:    167.8K
 * Total Submissions: 584K
 * Testcase Example:  '1'
 *
 * Given a positive integer, return its corresponding column title as appear in
 * an Excel sheet.
 * 
 * For example:
 * 
 * 
 * ⁠   1 -> A
 * ⁠   2 -> B
 * ⁠   3 -> C
 * ⁠   ...
 * ⁠   26 -> Z
 * ⁠   27 -> AA
 * ⁠   28 -> AB 
 * ⁠   ...
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: 1
 * Output: "A"
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 28
 * Output: "AB"
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: 701
 * Output: "ZY"
 * 
 */
impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        let mut n = n as u32;
        let mut result: Vec<u8> = vec![];
        let letters: Vec<_> = (b'A'..=b'Z').collect();
        let letter_count = letters.len() as u32;
        while n > 0 {
            n -= 1;
            result.push(letters[(n % letter_count) as usize]);
            n /= letter_count;
        }
        result.reverse();
        String::from_utf8(result).unwrap()

    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::convert_to_title(28), "AB".to_owned());
        assert_eq!(Solution::convert_to_title(1), "A".to_owned());
    }
}