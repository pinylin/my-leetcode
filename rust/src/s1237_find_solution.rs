/*
 * @lc app=leetcode.cn id=1237 lang=rust
 *
 * [1237] 找出给定方程的正整数解
 */

pub struct CustomFunction;
impl CustomFunction {
    pub fn f(&self, x: i32, y: i32) -> i32 {
        x * y
    }
}

// @lc code=start
/*
 * // This is the custom function interface.
 * // You should not implement it, or speculate about its implementation
 * struct CustomFunction;
 * impl CustomFunction {
 *    pub fn f(x:i32,y:i32)->i32{}
 * }
 */

impl Solution {
    pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        let (mut x, mut y) = (1, 1000);
        let mut res = Vec::new();
        while x < 1001 && y > 0 {
            match customfunction.f(x, y).cmp(&z) {
                std::cmp::Ordering::Greater => y -= 1,
                std::cmp::Ordering::Less => x += 1,
                std::cmp::Ordering::Equal => {
                    res.push(vec![x, y]);
                    y -= 1;
                    x += 1;
                }
            }
        }
        res
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::CustomFunction;
    use super::Solution;
    #[test]
    fn it_works() {
        let cf = CustomFunction;
        assert_eq!(
            Solution::find_solution(&cf, 5),
            vec![vec![1, 5], vec![5, 1]]
        );
    }
}
