/*
 * @lc app=leetcode id=171 lang=rust
 *
 * [171] Excel Sheet Column Number
 *
 * https://leetcode.com/problems/excel-sheet-column-number/description/
 *
 * algorithms
 * Easy (50.90%)
 * Total Accepted:    213.6K
 * Total Submissions: 417.3K
 * Testcase Example:  '"A"'
 *
 * Given a column title as appear in an Excel sheet, return its corresponding
 * column number.
 *
 * For example:
 *
 *
 * ⁠   A -> 1
 * ⁠   B -> 2
 * ⁠   C -> 3
 * ⁠   ...
 * ⁠   Z -> 26
 * ⁠   AA -> 27
 * ⁠   AB -> 28
 * ⁠   ...
 *
 *
 * Example 1:
 *
 *
 * Input: "A"
 * Output: 1
 *
 *
 * Example 2:
 *
 *
 * Input: "AB"
 * Output: 28
 *
 *
 * Example 3:
 *
 *
 * Input: "ZY"
 * Output: 701
 *
 */
impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let sv = s.as_bytes();
        let mut res = 0;
        for i in 0..sv.len() {
            res += (sv[i] - 64) as i32 * i32::pow(26, (sv.len() - i - 1) as u32);
        }
        res as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::title_to_number("A".to_owned()), 1);
        assert_eq!(Solution::title_to_number("AB".to_owned()), 28);
    }
}
