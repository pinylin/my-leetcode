/*
 * @lc app=leetcode.cn id=1436 lang=rust
 *
 * [1436] 旅行终点站
 */

// @lc code=start
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let sets: std::collections::HashSet<&String> = paths.iter().map(|x| &x[0]).collect();
        paths.iter().find(|x| !sets.contains(&x[1])).unwrap()[1].clone()
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
            Solution::dest_city(vec![
                vec_string!("B", "C"),
                vec_string!("D", "B"),
                vec_string!("C", "A")
            ]),
            "A".to_owned()
        )
    }
}
