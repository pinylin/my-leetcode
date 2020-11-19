/*
 * @lc app=leetcode.cn id=1460 lang=rust
 *
 * [1460] 通过翻转子数组使两个数组相等
 */

// @lc code=start
impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut target = target;
        let mut arr = arr;
        target.sort_unstable();
        arr.sort_unstable();
        target == arr
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]),
            true
        )
    }
}
