/*
 * @lc app=leetcode.cn id=1566 lang=rust
 *
 * [1566] 重复至少 K 次且长度为 M 的模式
 */

// @lc code=start
impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let (m, k) = (m as usize, k as usize);
        arr.windows(m * k)
            .any(|window| window.chunks(m).all(|chunk| chunk == &window[0..m]))
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
        assert_eq!(
            Solution::contains_pattern(vec![1, 2, 4, 4, 4, 4], 1, 3),
            true
        );
        assert_eq!(
            Solution::contains_pattern(vec![1, 2, 1, 2, 1, 1, 1, 3], 2, 2),
            true
        );
        assert_eq!(Solution::contains_pattern(vec![1, 2, 3, 1, 2], 2, 2), false);
    }
}
