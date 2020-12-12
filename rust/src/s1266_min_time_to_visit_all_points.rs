/*
 * @lc app=leetcode.cn id=1266 lang=rust
 *
 * [1266] 访问所有点的最小时间
 */

// @lc code=start
impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut steps = 0;
        for i in 1..points.len() {
            steps += std::cmp::max(
                (points[i][0] - points[i - 1][0]).abs(),
                (points[i][1] - points[i - 1][1]).abs(),
            );
        }
        steps

        // 方法2  参考 1566 注意windows和chunks不同
        // points
        //     .windows(2)
        //     .map(|x| std::cmp::max((x[1][0] - x[0][0]).abs(), (x[1][1] - x[0][1]).abs()))
        //     .sum()
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
            Solution::min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0],]),
            7
        );
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2],]),
            5
        );
    }
}
