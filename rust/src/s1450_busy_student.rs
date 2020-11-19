/*
 * @lc app=leetcode.cn id=1450 lang=rust
 *
 * [1450] 在既定时间做作业的学生人数
 */

// @lc code=start
impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        (0..start_time.len()).fold(0, |sum, i| {
            sum + if start_time[i] <= query_time && query_time <= end_time[i] {
                1
            } else {
                0
            }
        })
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
            Solution::busy_student(
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
                vec![10, 10, 10, 10, 10, 10, 10, 10, 10],
                5
            ),
            5
        )
    }
}
