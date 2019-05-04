/*
 * @lc app=leetcode id=453 lang=rust
 *
 * [453] Minimum Moves to Equal Array Elements
 */
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min = nums[0];
        for &i in nums.iter() {
            sum += i;
            if i < min {
                min = i;
            } 
        }
        sum - min * nums.len() as i32
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::min_moves(vec![1,2,3]),
            3
        );
    }
}
