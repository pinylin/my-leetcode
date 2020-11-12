/*
 * @lc app=leetcode.cn id=1512 lang=rust
 *
 * [1512] 好数对的数目
 */

// @lc code=start
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut counter = vec![0; 101];
        for i in nums {
            counter[i as usize] += 1;
        }
        let mut total = 0;
        for n in counter {
            total += (n - 1) * n / 2;
        }
        total
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
        assert_eq!(Solution::num_identical_pairs(vec![1,1,1,1]), 6);
    }
}

