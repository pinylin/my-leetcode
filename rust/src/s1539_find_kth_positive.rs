/*
 * @lc app=leetcode.cn id=1539 lang=rust
 *
 * [1539] 第 k 个缺失的正整数
 */

// @lc code=start
impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut count = 1;
        let mut idx: usize = 0;
        while k > 0 {
            if idx < arr.len() && count == arr[idx] {
                idx += 1;
            } else {
                k -= 1;
            }
            count += 1;
        }
        count - 1
    }
}
// @lc code=end
pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    // use crate::utils::vec2d;
    #[test]
    fn it_works() {
        assert_eq!(Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
    }
}
