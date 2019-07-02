/*
 * @lc app=leetcode id=493 lang=rust
 *
 * [493] Reverse Pairs
 */

impl Solution {
    /// Time Limit Exceeded 从50000-1的testcase 看来mergesort还是不够好，还是要用binary index tree?
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        let nl = nums.len();
        if nl < 2 {
            return 0;
        }
        Solution::merge_helper(nums.as_mut(), 0, nl - 1)
    }
    fn merge_helper(nums: &mut Vec<i32>, left: usize, right: usize) -> i32 {
        if left >= right {
            return 0;
        }
        let mid = left + (right - left) / 2;
        let mut res =
            Solution::merge_helper(nums, left, mid) + Solution::merge_helper(nums, mid + 1, right);
        for i in left..=mid {
            let mut j = mid + 1;
            while j <= right && nums[i] as f32 / 2.0 > nums[j] as f32 {
                j += 1;
            }
            res += (j - (mid + 1)) as i32;
        }
        Solution::merge(nums, left, mid, right);
        res
    }
    fn merge(a: &mut Vec<i32>, b: usize, m: usize, e: usize) {
        let mut left = a[b..=m].to_vec();
        let mut right = a[m + 1..=e].to_vec();
        left.reverse();
        right.reverse();
        for k in b..=e {
            if left.is_empty() {
                a[k] = right.pop().unwrap();
                continue;
            }
            if right.is_empty() {
                a[k] = left.pop().unwrap();
                continue;
            }
            if right.last() < left.last() {
                a[k] = right.pop().unwrap();
            } else {
                a[k] = left.pop().unwrap();
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse_pairs(vec![]), 0);
        assert_eq!(Solution::reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
        assert_eq!(Solution::reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
        assert_eq!(
            Solution::reverse_pairs(vec![
                2147483647, 2147483647, 2147483647, 2147483647, 2147483647, 2147483647
            ]),
            0
        )
    }
}
