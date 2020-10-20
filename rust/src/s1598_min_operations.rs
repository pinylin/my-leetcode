/*
 * @lc app=leetcode.cn id=1598 lang=rust
 *
 * [1598] 文件夹操作日志搜集器
 */

// @lc code=start
impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut depth = 0;
        for item in logs.iter() {
            match item.as_str() {
                "./" => {}
                "../" => {
                    depth = 0.max(depth - 1);
                }
                _ => depth += 1,
            }
        }
        depth as i32
    }
}
// @lc code=end

pub struct Solution;
mod test {
    use super::Solution;
    use crate::vec_string;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::min_operations(vec_string!["d1/", "d2/", "../", "d21/", "./"]),
            2
        );
        assert_eq!(
            Solution::min_operations(vec_string!["d1/", "d2/", "./", "d3/", "../", "d31/"]),
            3
        );
        assert_eq!(
            Solution::min_operations(vec_string!["d1/", "../", "../", "../"]),
            0
        );
    }
}
