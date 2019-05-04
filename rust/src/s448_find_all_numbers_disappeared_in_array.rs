/*
 * @lc app=leetcode id=448 lang=rust
 *
 * [448] Find All Numbers Disappeared in an Array
 */
#![allow(dead_code)]
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut mark: Vec<bool> = vec![true; nums.len() + 1];
        let mut res = vec![];
        for num in nums {
            mark[num as usize] = false;
        }
        for (idx, item) in mark.iter().enumerate().skip(1) {
            if *item {
                res.push(idx as i32);
            }
        }
        res
    }

    pub fn find_disappeared_numbers_2(mut nums: Vec<i32>) -> Vec<i32> {
        let count = nums.len();
        nums.sort();
        nums.dedup();
        let mut res = vec![];
        let mut it = 0;
        for item in 1..=count as i32 {
            if it >= nums.len() {
                res.push(item);
                continue;
            }
            if nums[it] != item {
                res.push(item);
            } else {
                it += 1;
            }
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
        assert_eq!(
            Solution::find_disappeared_numbers_best(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
    }
}
