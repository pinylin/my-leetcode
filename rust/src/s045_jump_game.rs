/*
 * @lc app=leetcode.cn id=45 lang=rust
 *
 * [45] 跳跃游戏 II
 */

// @lc code=start
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut step = 0;
        let mut right = 1;
        let mut farthest = 0;
        let mut i = 0;

        while right < nums.len() {
            farthest = farthest.max(i + nums[i] as usize);

            i += 1;

            if i == right {
                step += 1;

                right = farthest + 1;
            }
        }

        step
    }
    // reach time limit
    pub fn jump_dfs(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return nums.len() as i32 - 1;
        }
        Self::dfs(&nums)
    }

    pub fn dfs(nums: &[i32]) -> i32 {
        let res = 1;
        if nums[0] == 0 {
            return i32::MAX;
        }
        if nums[0] as usize >= nums.len() - 1 {
            return res;
        }

        let mut cur = nums[0].clone();
        let mut step = i32::MAX;
        while cur > 0 {
            step = step.min(Self::dfs(&nums[cur as usize..]));
            cur -= 1;
        }
        res.saturating_add(step)
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![0]), 0);
        assert_eq!(Solution::jump(vec![5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0]), 3);
        assert_eq!(
            Solution::jump(vec![
                5, 6, 4, 4, 6, 9, 4, 4, 7, 4, 4, 8, 2, 6, 8, 1, 5, 9, 6, 5, 2, 7, 9, 7, 9, 6, 9, 4,
                1, 6, 8, 8, 4, 4, 2, 0, 3, 8, 5
            ]),
            5
        );
    }
}
