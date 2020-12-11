/*
 * @lc app=leetcode.cn id=1313 lang=rust
 *
 * [1313] 解压缩编码列表
 */

// @lc code=start
impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();

        for idx in (0..nums.len()).step_by(2) {
            res.extend(&vec![nums[idx + 1]; nums[idx] as usize]);
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
        assert_eq!(
            Solution::decompress_rl_elist(vec![1, 2, 3, 4]),
            [2, 4, 4, 4]
        );
        assert_eq!(Solution::decompress_rl_elist(vec![1, 1, 2, 3]), [1, 3, 3]);
    }
}
