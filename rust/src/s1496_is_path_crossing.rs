/*
 * @lc app=leetcode.cn id=1496 lang=rust
 *
 * [1496] 判断路径是否相交
 */

// @lc code=start
impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut track = std::collections::HashSet::new();
        let mut pos = (0, 0);
        track.insert(pos);
        for direction in path.chars() {
            match direction {
                'N' => pos.0 += 1,
                'S' => pos.0 -= 1,
                'E' => pos.1 += 1,
                'W' => pos.1 -= 1,
                _ => (),
            };
            if !track.insert(pos) {
                return true;
            }
        }
        false
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
        assert_eq!(Solution::is_path_crossing("NESWW".to_owned()), true);
    }
}
