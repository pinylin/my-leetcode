/*
 * @lc app=leetcode.cn id=1582 lang=rust
 *
 * [1582] 二进制矩阵中的特殊位置
 */

// @lc code=start
impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let (rn, cn) = (mat.len(), mat[0].len());
        let mut row = vec![0; rn];
        let mut col = vec![0; cn];
        let mut res = 0;
        for i in 0..rn {
            for j in 0..cn {
                row[i] += mat[i][j];
                col[j] += mat[i][j];
            }
        }
        for i in 0..rn {
            for j in 0..cn {
                if mat[i][j] == 1 && row[i] == 1 && col[j] == 1 {
                    res += 1;
                }
            }
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
        assert_eq!(
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
    }
}
