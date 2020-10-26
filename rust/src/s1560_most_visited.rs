/*
 * @lc app=leetcode.cn id=1560 lang=rust
 *
 * [1560] 圆形赛道上经过次数最多的扇区
 */

// @lc code=start
impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let start = *rounds.first().unwrap();
        let end = *rounds.last().unwrap();

        if start <= end {
            (start..=end).collect()
        } else {
            (1..=end).chain(start..=n).collect()
        }
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
        assert_eq!(Solution::most_visited(4, vec![1, 3, 1, 2]), vec![1, 2]);
        assert_eq!(
            Solution::most_visited(2, vec![2, 1, 2, 1, 2, 1, 2, 1, 2]),
            vec![2]
        );
        assert_eq!(
            Solution::most_visited(7, vec![1, 3, 5, 7]),
            vec![1, 2, 3, 4, 5, 6, 7]
        );
    }
}
