/*
 * @lc app=leetcode id=349 lang=rust
 *
 * [349] Intersection of Two Arrays
 */
impl Solution {
    pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut nums = Vec::new();
        nums1.sort();
        nums2.sort();
        let mut i = 0;
        let mut j = 0;
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] > nums2[j] {
                j += 1;
                continue;
            }
            if nums1[i] == nums2[j] {
                nums.push(nums1[i]);
                i += 1;
                j += 1;
                continue;
            }
            if nums1[i] < nums2[j] {
                i += 1;
                continue;
            }
        }
        nums
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::intersection(vec![1, 9, 2, 4, 9], vec![3, 5, 9, 8, 7, 9]),
            vec![9, 9]
        );
    }
}
