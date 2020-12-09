/*
 * @lc app=leetcode.cn id=1351 lang=rust
 *
 * [1351] 统计有序矩阵中的负数
 */

// @lc code=start
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut col = grid[0].len() as i32 - 1;
        for r in 0..grid.len() {
            while col >= 0 && grid[r][col as usize] < 0 {
                col -= 1;
            }
            res += grid[0].len() as i32 - col - 1;
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
            Solution::count_negatives(vec![vec![1, -1], vec![-1, -1]]),
            3
        );
        assert_eq!(
            Solution::count_negatives(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3]
            ]),
            8
        );
    }
}
