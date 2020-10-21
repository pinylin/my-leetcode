/*
 * @lc app=leetcode.cn id=1588 lang=rust
 *
 * [1588] 所有奇数长度子数组的和
 */

// @lc code=start
impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut res = 0;
        let size = arr.len();
        for i in 0..size {
            let (left, right) = (i + 1, size - i);
            let (left_even, right_even) = ((left + 1) / 2, (right + 1) / 2);
            let (left_odd, right_odd) = (left / 2, right / 2);
            res += (left_even * right_even + left_odd * right_odd) as i32 * arr[i];
        }
        res
    }
}
// @lc code=end

pub struct Solution;
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
    }
}
