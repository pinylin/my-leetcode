/*
 * @lc app=leetcode.cn id=1491 lang=rust
 *
 * [1491] 去掉最低工资和最高工资后的工资平均值
 */

// @lc code=start
impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        // 方法1
        // 排序，然后去掉头尾，求和再求平均数
        // Passed 0ms 2.1mb
        let mut salary = salary;
        salary.sort();
        (1..salary.len() - 1).fold(0_f64, |acc, i| acc + salary[i] as f64)
            / (salary.len() - 2) as f64

        // 方法2
        // 先求整体和，再减去最大值和最小值，再求平均数
        // (salary.iter().sum::<i32>()
        //     - *salary.iter().min().unwrap_or(&0)
        //     - *salary.iter().max().unwrap_or(&0)) as f64
        //     / (salary.len() - 2) as f64
    }
}
// @lc code=end
pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    // use crate::utils::vec2d;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::average(vec![8000, 9000, 2000, 3000, 6000, 1000]),
            4750.00000
        )
    }
}
