/*
 * @lc app=leetcode.cn id=575 lang=rust
 *
 * [575] 分糖果
 */

// @lc code=start
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let give = candy_type.len() / 2;
        let mut set = std::collections::HashSet::new();

        for candy in &candy_type {
            if !set.contains(&candy) {
                set.insert(candy);
            }
        }

        std::cmp::min(set.len(), give) as i32
    }
}
// @lc code=end

pub struct Solution {}
