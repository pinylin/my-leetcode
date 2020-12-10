/*
 * @lc app=leetcode.cn id=1337 lang=rust
 *
 * [1337] 方阵中战斗力最弱的 K 行
 */

// @lc code=start
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut x: Vec<(i32, usize)> = mat
            .iter()
            .enumerate()
            .map(|(i, x)| (x.iter().sum(), i))
            .collect();
        x.sort();
        let mut x: Vec<i32> = x.into_iter().map(|(_, i)| i as i32).collect();
        while x.len() > k as usize {
            x.pop();
        }
        x
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
            Solution::k_weakest_rows(
                vec![
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 1]
                ],
                3
            ),
            vec![2, 0, 3]
        );
    }
}
