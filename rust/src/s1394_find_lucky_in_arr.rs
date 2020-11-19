/*
 * @lc app=leetcode.cn id=1394 lang=rust
 *
 * [1394] 找出数组中的幸运数
 */

// @lc code=start
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut res = -1;
        let mut map = std::collections::HashMap::new();
        for item in arr.iter() {
            *map.entry(*item).or_insert(0) += 1;
        }
        for (k, v) in map {
            if k == v {
                res = std::cmp::max(res, v)
            }
        }
        res
    }
}
// @lc code=end
pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::find_lucky(vec![7, 7, 7, 7, 7, 7, 7]), 7);
        assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
    }
}
