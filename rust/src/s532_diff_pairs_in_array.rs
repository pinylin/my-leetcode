/*
 * @lc app=leetcode id=532 lang=rust
 *
 * [532] K-diff Pairs in an Array
 */
impl Solution {
    pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut j = 0;
        let mut i = 0;
        nums.sort();
        while i < nums.len() {
            j = std::cmp::max(j, i + 1);
            while j < nums.len() && nums[j] - nums[i] < k {
                j += 1;
            }
            if j < nums.len() && nums[j] - nums[i] == k {
                res += 1;
            }
            while i < nums.len() - 1 && nums[i] == nums[i + 1] {
                i += 1;
            }
            i += 1;
        }
        res
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::find_pairs(vec![3, 1, 4, 1, 5], 2), 2);
    }
}
