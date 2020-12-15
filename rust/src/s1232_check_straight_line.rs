/*
 * @lc app=leetcode.cn id=1232 lang=rust
 *
 * [1232] 缀点成线
 */

// @lc code=start
impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let (x0, y0, x1, y1) = (
            coordinates[0][0],
            coordinates[0][1],
            coordinates[1][0],
            coordinates[1][1],
        );
        for i in 2..coordinates.len() {
            let (x, y) = (coordinates[i][0], coordinates[i][1]);
            if (y - y0) * (x - x1) != (y - y1) * (x - x0) {
                return false;
            }
        }
        true
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
            Solution::check_straight_line(vec![
                vec![1, 1],
                vec![2, 2],
                vec![3, 4],
                vec![4, 5],
                vec![5, 6],
                vec![7, 7]
            ]),
            false
        );
        assert_eq!(
            Solution::check_straight_line(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 5],
                vec![5, 6],
                vec![6, 7]
            ]),
            true
        );
    }
}
