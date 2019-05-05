/*
 * @lc app=leetcode id=463 lang=rust
 *
 * [463] Island Perimeter
 */
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }
        let mut res = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    continue;
                }
                res += 4;
                if i > 0 && grid[i - 1][j] == 1 {
                    res -= 2;
                }
                if j > 0 && grid[i][j - 1] == 1 {
                    res -= 2;
                }
            }
        }

        res
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ]),
            16
        );
    }
}
