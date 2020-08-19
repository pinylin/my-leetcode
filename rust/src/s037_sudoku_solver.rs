/*
 * @lc app=leetcode id=37 lang=rust
 *
 * [37] Sudoku Solver
 *
 * https://leetcode.com/problems/sudoku-solver/description/
 *
 * algorithms
 * Hard (42.96%)
 * Likes:    1937
 * Dislikes: 92
 * Total Accepted:    192.7K
 * Total Submissions: 440.9K
 * Testcase Example:  '[["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]'
 *
 * Write a program to solve a Sudoku puzzle by filling the empty cells.
 *
 * A sudoku solution must satisfy all of the following rules:
 *
 *
 * Each of the digits 1-9 must occur exactly once in each row.
 * Each of the digits 1-9 must occur exactly once in each column.
 * Each of the the digits 1-9 must occur exactly once in each of the 9 3x3
 * sub-boxes of the grid.
 *
 *
 * Empty cells are indicated by the character '.'.
 *
 *
 * A sudoku puzzle...
 *
 *
 * ...and its solution numbers marked in red.
 *
 * Note:
 *
 *
 * The given board contain only digits 1-9 and the character '.'.
 * You may assume that the given Sudoku puzzle will have a single unique
 * solution.
 * The given board size is always 9x9.
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Solution::helper(board, 0, 0);
    }
    fn helper(board: &mut Vec<Vec<char>>, i: usize, j: usize) -> bool {
        if i == 9 {
            return true;
        }
        if j >= 9 {
            return Solution::helper(board, i + 1, 0);
        }
        if board[i][j] != '.' {
            return Solution::helper(board, i, j + 1);
        }
        for c in 1..=9 {
            if Solution::is_valid(&board, i, j, (c + '0' as u8) as char) {
                board[i][j] = (c + '0' as u8) as char;
                if Solution::helper(board, i, j + 1) {
                    return true;
                } else {
                    board[i][j] = '.';
                }
            }
        }
        return false;
    }

    fn is_valid(board: &Vec<Vec<char>>, x: usize, y: usize, c: char) -> bool {
        for z in 0..9 {
            if board[x][z] == c
                || board[z][y] == c
                || board[3 * (x / 3) + z / 3][3 * (y / 3) + z % 3] == c
            {
                return false;
            }
        }
        true
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let res = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(res, board);
    }
}
