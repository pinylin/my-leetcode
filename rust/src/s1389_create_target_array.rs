/*
 * @lc app=leetcode.cn id=1389 lang=rust
 *
 * [1389] 按既定顺序创建目标数组
 */

// @lc code=start
impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(index.len());
        for (k, v) in index.iter().zip(nums.iter()) {
            result.insert(*k as usize, *v);
        }
        result
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
            Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
            vec![0, 4, 1, 3, 2]
        );
        assert_eq!(
            Solution::create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
            vec![0, 1, 2, 3, 4]
        );
    }
}
