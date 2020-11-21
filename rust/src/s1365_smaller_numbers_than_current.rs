/*
 * @lc app=leetcode.cn id=1365 lang=rust
 *
 * [1365] 有多少小于当前数字的数字
 */

// @lc code=start
impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        for (i, n) in sorted_nums.iter().enumerate() {
            map.entry(*n).or_insert(i as i32);
        }
        nums.iter().map(|x| *map.get(&x).unwrap()).collect()
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        )
    }
}
