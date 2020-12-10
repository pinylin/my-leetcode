/*
 * @lc app=leetcode.cn id=1342 lang=rust
 *
 * [1342] 将数字变成 0 的操作次数
 */

// @lc code=start
impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut res = 0;
        while num > 0 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num -= 1;
            }
            res += 1;
        }
        res
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::number_of_steps(8), 4);
        assert_eq!(Solution::number_of_steps(14), 6);
        assert_eq!(Solution::number_of_steps(123), 12);
    }
}
