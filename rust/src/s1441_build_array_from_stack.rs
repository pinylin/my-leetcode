/*
 * @lc app=leetcode.cn id=1441 lang=rust
 *
 * [1441] 用栈操作构建数组
 */

// @lc code=start
impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut result = vec![];
        let push: String = "Push".to_string();
        let pop: String = "Pop".to_string();
        let mut cur = 0;
        (1..=n).for_each(|i| {
            if cur < target.len() {
                if i < target[cur] {
                    result.push(push.clone());
                    result.push(pop.clone());
                } else {
                    result.push(push.clone());
                    cur += 1
                }
            }
        });

        result
    }
}
// @lc code=end
pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    use crate::vec_string;
    // use crate::utils::vec2d;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::build_array(vec![1, 3], 3),
            vec_string!["Push", "Push", "Pop", "Push"]
        );
        assert_eq!(
            Solution::build_array(vec![2, 3, 4], 4),
            vec_string!["Push", "Pop", "Push", "Push", "Push"]
        )
    }
}
