/*
 * @lc app=leetcode.cn id=1304 lang=rust
 *
 * [1304] 和为零的N个唯一整数
 */

// @lc code=start
impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        if n % 2 == 1 {
            res.push(0);
        }
        for i in 1..n / 2 + 1 {
            res.push(i);
            res.push(i * -1);
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
        assert_eq!(Solution::sum_zero(1).iter().sum::<i32>(), 0);
        assert_eq!(Solution::sum_zero(5).iter().sum::<i32>(), 0);
    }
}
