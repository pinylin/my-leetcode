/*
 * @lc app=leetcode.cn id=1470 lang=rust
 *
 * [1470] 重新排列数组
 */

// @lc code=start
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = Vec::with_capacity(2 * n);
        for i in 0..2 * n {
            if i % 2 == 0 {
                res.push(nums[i / 2]);
            } else {
                res.push(nums[n + i / 2]);
            }
        }
        return res;
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
            Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            vec![1, 4, 2, 3, 3, 2, 4, 1]
        )
    }
}
