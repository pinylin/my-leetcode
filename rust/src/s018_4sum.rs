/*
 * @lc app=leetcode id=18 lang=rust
 *
 * [18] 4Sum
 */

// @lc code=start
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let len = nums.len();
        if len < 4 {
            return vec![];
        }

        nums.sort();

        let mut found = vec![];
        let mut pre: i32 = std::i32::MAX;

        for i in 0..(len - 3) {
            if pre == nums[i] {
                continue;
            }
            pre = nums[i];

            let three = Solution::three_sum(&nums[i + 1..], target - nums[i]);
            for item in three {
                found.push(vec![nums[i], item[0], item[1], item[2]]);
            }
        }

        found
    }

    pub fn three_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut found = vec![];
        let mut pre: i32 = std::i32::MAX;

        if nums[0] + nums[1] + nums[2] > target {
            return found;
        }
        if nums[len - 1] + nums[len - 2] + nums[len - 3] < target {
            return found;
        }

        for i in 0..(len - 2) {
            if pre == nums[i] {
                continue;
            }
            pre = nums[i];

            let two = Solution::two_sum(&nums[i + 1..], target - nums[i]);
            for item in two {
                found.push(vec![nums[i], item[0], item[1]]);
            }
        }

        found
    }

    pub fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut found = vec![];
        let mut pre: i32 = std::i32::MAX;

        if nums[0] + nums[1] > target {
            return found;
        }
        if nums[len - 1] + nums[len - 2] < target {
            return found;
        }

        for i in 0..(len - 1) {
            if pre == nums[i] {
                continue;
            }
            pre = nums[i];

            if let Some(one) = Solution::one(&nums[i + 1..], target - nums[i]) {
                found.push(vec![nums[i], one]);
            }
        }

        found
    }

    pub fn one(nums: &[i32], target: i32) -> Option<i32> {
        let len = nums.len();
        if len == 0 || target < nums[0] || target > nums[len - 1] {
            return None;
        }
        let mid_index = (len - 1) / 2;
        let mid = nums[mid_index];
        if target < mid {
            return Solution::one(&nums[..mid_index], target);
        } else if target > mid {
            return Solution::one(&nums[mid_index + 1..], target);
        } else {
            return Some(target);
        }
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
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );

        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 3),
            vec![vec![0, 0, 1, 2]]
        );

        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0], 0),
            vec![vec![-1, 0, 0, 1],]
        );

        let res: Vec<Vec<i32>> = Vec::new();
        assert_eq!(Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 30), res);

        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, -1, -1, -1, -1], -4),
            vec![vec![-1, -1, -1, -1]]
        );

        assert_eq!(
            Solution::four_sum(vec![-5, -4, -3, -2, -1, 0, 0, 1, 2, 3, 4, 5], 0),
            vec![
                vec![-5, -4, 4, 5],
                vec![-5, -3, 3, 5],
                vec![-5, -2, 2, 5],
                vec![-5, -2, 3, 4],
                vec![-5, -1, 1, 5],
                vec![-5, -1, 2, 4],
                vec![-5, 0, 0, 5],
                vec![-5, 0, 1, 4],
                vec![-5, 0, 2, 3],
                vec![-4, -3, 2, 5],
                vec![-4, -3, 3, 4],
                vec![-4, -2, 1, 5],
                vec![-4, -2, 2, 4],
                vec![-4, -1, 0, 5],
                vec![-4, -1, 1, 4],
                vec![-4, -1, 2, 3],
                vec![-4, 0, 0, 4],
                vec![-4, 0, 1, 3],
                vec![-3, -2, 0, 5],
                vec![-3, -2, 1, 4],
                vec![-3, -2, 2, 3],
                vec![-3, -1, 0, 4],
                vec![-3, -1, 1, 3],
                vec![-3, 0, 0, 3],
                vec![-3, 0, 1, 2],
                vec![-2, -1, 0, 3],
                vec![-2, -1, 1, 2],
                vec![-2, 0, 0, 2],
                vec![-1, 0, 0, 1]
            ]
        );

        assert_eq!(
            Solution::four_sum(vec![0, 0, 0, 0, 0], 0),
            vec![vec![0, 0, 0, 0]]
        );
    }
}
