/*
 * @lc app=leetcode.cn id=1346 lang=rust
 *
 * [1346] 检查整数及其两倍数是否存在
 */

// @lc code=start
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        // let sets: std::collections::HashSet<i32> = arr.iter().map(|x| *x * 2).collect();
        let mut sets = std::collections::HashSet::new();
        for x in arr {
            if sets.contains(&(x * 2)) || (x % 2 == 0 && sets.contains(&(x / 2))) {
                // 当前数*2是否存在  or 当前数是偶数，那么它的一半是否存在
                return true;
            }
            sets.insert(x);
        }
        false
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::check_if_exist(vec![3, 1, 7, 11]), false);
        assert_eq!(Solution::check_if_exist(vec![10, 2, 5, 3]), true);
        assert_eq!(Solution::check_if_exist(vec![7, 1, 14, 11]), true);
        assert_eq!(
            Solution::check_if_exist(vec![-2, 0, 10, -19, 4, 6, -8]),
            false
        );
        assert_eq!(Solution::check_if_exist(vec![-2, 0, 4]), false);
        assert_eq!(Solution::check_if_exist(vec![-2, 0, 0]), true);
    }
}
