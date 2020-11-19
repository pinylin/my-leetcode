/*
 * @lc app=leetcode.cn id=1403 lang=rust
 *
 * [1403] 非递增顺序的最小子序列
 */

// @lc code=start
impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        nums.reverse();
        let total: i32 = nums.iter().sum();
        let (mut sum, mut res) = (0, Vec::new());
        for x in nums {
            if sum > total - sum {
                break;
            }
            res.push(x);
            sum += x;
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
        assert_eq!(Solution::min_subsequence(vec![4, 3, 10, 9, 8]), vec![10, 9]);
        assert_eq!(
            Solution::min_subsequence(vec![4, 4, 7, 6, 7]),
            vec![7, 7, 6]
        );
    }
}
