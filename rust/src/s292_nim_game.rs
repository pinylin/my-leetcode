/*
 * @lc app=leetcode id=292 lang=rust
 *
 * [292] Nim Game
 */
impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        if n % 4 == 0 {
            return false;
        }
        true
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::can_win_nim(4), false);
    }
}
