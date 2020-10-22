/*
 * @lc app=leetcode.cn id=1572 lang=rust
 *
 * [1572] 矩阵对角线元素的和
 */

// @lc code=start
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let rn = mat.len();
        let mut res = 0;
        for i in 0..rn {
            res += mat[i][i];
            res += mat[rn - i - 1][i];
        }
        if rn % 2 != 0 {
            let mid = rn / 2;
            res -= mat[mid][mid];
        }

        res
    }
}
// @lc code=end
pub struct Solution;
