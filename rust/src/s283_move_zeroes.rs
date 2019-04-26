/*
 * @lc app=leetcode id=283 lang=rust
 *
 * [283] Move Zeroes
 */
impl Solution {
    // review
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(i, j);
                j += 1;
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
        let mut nums = vec![0, 1, 0, 3, 12, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(
            nums, 
            vec![1, 3, 12, 0, 0, 0]
        );
    }
}