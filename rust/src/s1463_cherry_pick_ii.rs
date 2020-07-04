/*
 * @lc app=leetcode id=1463 lang=rust
 *
 * [1463] Cherry Pickup II
 *
 * https://leetcode.com/problems/cherry-pickup-ii/description/
 *
 * algorithms
 * Hard (63.82%)
 * Likes:    169
 * Dislikes: 3
 * Total Accepted:    4.9K
 * Total Submissions: 7.6K
 * Testcase Example:  '[[3,1,1],[2,5,1],[1,5,5],[2,1,1]]'
 *
 * Given a rows x cols matrix grid representing a field of cherries. Each cell
 * in grid represents the number of cherries that you can collect.
 *
 * You have two robots that can collect cherries for you, Robot #1 is located
 * at the top-left corner (0,0) , and Robot #2 is located at the top-right
 * corner (0, cols-1) of the grid.
 *
 * Return the maximum number of cherries collection using both robots  by
 * following the rules below:
 *
 *
 * From a cell (i,j), robots can move to cell (i+1, j-1) , (i+1, j) or (i+1,
 * j+1).
 * When any robot is passing through a cell, It picks it up all cherries, and
 * the cell becomes an empty cell (0).
 * When both robots stay on the same cell, only one of them takes the
 * cherries.
 * Both robots cannot move outside of the grid at any moment.
 * Both robots should reach the bottom row in the grid.
 *
 *
 *
 * Example 1:
 *
 *
 *
 *
 * Input: grid = [[3,1,1],[2,5,1],[1,5,5],[2,1,1]]
 * Output: 24
 * Explanation: Path of robot #1 and #2 are described in color green and blue
 * respectively.
 * Cherries taken by Robot #1, (3 + 2 + 5 + 2) = 12.
 * Cherries taken by Robot #2, (1 + 5 + 5 + 1) = 12.
 * Total of cherries: 12 + 12 = 24.
 *
 *
 * Example 2:
 *
 *
 *
 *
 * Input: grid =
 * [[1,0,0,0,0,0,1],[2,0,0,0,0,3,0],[2,0,9,0,0,0,0],[0,3,0,5,4,0,0],[1,0,2,3,0,0,6]]
 * Output: 28
 * Explanation: Path of robot #1 and #2 are described in color green and blue
 * respectively.
 * Cherries taken by Robot #1, (1 + 9 + 5 + 2) = 17.
 * Cherries taken by Robot #2, (1 + 3 + 4 + 3) = 11.
 * Total of cherries: 17 + 11 = 28.
 *
 *
 * Example 3:
 *
 *
 * Input: grid = [[1,0,0,3],[0,0,0,3],[0,0,3,3],[9,0,3,3]]
 * Output: 22
 *
 *
 * Example 4:
 *
 *
 * Input: grid = [[1,1],[1,1]]
 * Output: 4
 *
 *
 *
 * Constraints:
 *
 *
 * rows == grid.length
 * cols == grid[i].length
 * 2 <= rows, cols <= 70
 * 0 <= grid[i][j] <= 100
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp1 = vec![vec![-1; n]; n];
        let mut dp2 = vec![vec![-1; n]; n];
        dp1[0][n - 1] = grid[0][0] + grid[0][n - 1];
        for r in 1..m {
            for i in 0..n {
                for j in 0..n {
                    let mut t = -1;
                    for x in 0..=2 {
                        for y in 0..=2 {
                            if (i == 0 && x == 0)
                                || (j == 0 && y == 0)
                                || i + x - 1 >= n
                                || j + y - 1 >= n
                            {
                                continue;
                            }
                            t = std::cmp::max(t, dp1[i + x - 1][j + y - 1]);
                        }
                    }
                    if t < 0 {
                        continue;
                    }
                    dp2[i][j] = t + if i == j {
                        grid[r][i]
                    } else {
                        grid[r][i] + grid[r][j]
                    };
                }
            }
            std::mem::swap(&mut dp1, &mut dp2)
        }
        dp1.iter().map(|v| *v.iter().max().unwrap()).max().unwrap()
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
            Solution::cherry_pickup(vec![
                vec![3, 1, 1],
                vec![2, 5, 1],
                vec![1, 5, 5],
                vec![2, 1, 1]
            ]),
            24
        );
        assert_eq!(
            Solution::cherry_pickup(vec![
                vec![1, 0, 0, 0, 0, 0, 1],
                vec![2, 0, 0, 0, 0, 3, 0],
                vec![2, 0, 9, 0, 0, 0, 0],
                vec![0, 3, 0, 5, 4, 0, 0],
                vec![1, 0, 2, 3, 0, 0, 6]
            ]),
            28
        );
        assert_eq!(
            Solution::cherry_pickup(vec![
                vec![1, 0, 0, 3],
                vec![0, 0, 0, 3],
                vec![0, 0, 3, 3],
                vec![9, 0, 3, 3]
            ]),
            22
        );
        assert_eq!(Solution::cherry_pickup(vec![vec![1, 1], vec![1, 1]]), 4);
    }
}
