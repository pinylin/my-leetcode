/*
 * @lc app=leetcode.cn id=1217 lang=rust
 *
 * [1217] 玩筹码
 */

// @lc code=start
impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        // let odd = (position.len() + 1) / 2;
        // 好好审题，position中是筹码的位置
        let odd = position.iter().filter(|&p| p % 2 == 1).count();
        std::cmp::min(odd, position.len() - odd) as i32
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::min_cost_to_move_chips(vec![1, 2, 3]), 1);
        assert_eq!(Solution::min_cost_to_move_chips(vec![2, 2, 2, 3, 3]), 2);
        assert_eq!(
            Solution::min_cost_to_move_chips(vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30
            ]),
            15
        );
    }
}
