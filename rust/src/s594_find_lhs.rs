/*
 * @lc app=leetcode.cn id=594 lang=rust
 *
 * [594] 最长和谐子序列
 */

// @lc code=start
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut res = 0;
        for &num in nums.iter() {
            *map.entry(num).or_insert(0) += 1;
        }

        for &num in nums.iter() {
            let pre = map.get(&(num - 1));
            let cur = map.get(&num).unwrap();
            if pre.is_some() {
                if cur + *pre.unwrap() > res {
                    res = cur + *pre.unwrap();
                }
            }
        }

        res
    }
}
// @lc code=end

pub struct Solution {}
