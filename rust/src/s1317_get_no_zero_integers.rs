/*
 * @lc app=leetcode.cn id=1317 lang=rust
 *
 * [1317] 将整数转换为两个无零整数的和
 */

// @lc code=start
impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let has_zero = |x: i32| x.to_string().contains('0');

        for i in 1..n {
            if !has_zero(i) && !has_zero(n - i) {
                return vec![i, n - i];
            }
        }
        vec![-1, -1]
    }
}
// @lc code=end
pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::get_no_zero_integers(2), vec![1, 1]);
        assert_eq!(Solution::get_no_zero_integers(11), vec![2, 9]);
        assert_eq!(Solution::get_no_zero_integers(10000), vec![1, 9999]);
    }
}
