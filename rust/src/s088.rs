/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 *
 * https://leetcode.com/problems/merge-sorted-array/description/
 *
 * algorithms
 * Easy (34.91%)
 * Total Accepted:    340.9K
 * Total Submissions: 970.7K
 * Testcase Example:  '[1,2,3,0,0,0]\n3\n[2,5,6]\n3'
 *
 * Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as
 * one sorted array.
 *
 * Note:
 *
 *
 * The number of elements initialized in nums1 and nums2 are m and n
 * respectively.
 * You may assume that nums1 has enough space (size that is greater or equal to
 * m + n) to hold additional elements from nums2.
 *
 *
 * Example:
 *
 *
 * Input:
 * nums1 = [1,2,3,0,0,0], m = 3
 * nums2 = [2,5,6],       n = 3
 *
 * Output: [1,2,2,3,5,6]
 *
 *
 */
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut m, mut n) = (m - 1, n - 1);
        for i in (0..nums1.len()).rev() {
            let a = nums1.get(m as usize).unwrap_or(&std::i32::MIN);
            let b = nums2.get(n as usize).unwrap_or(&std::i32::MIN);
            if a > b {
                nums1[i] = *a;
                m -= 1;
            } else {
                nums1[i] = *b;
                n -= 1;
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
        let mut v1 = vec![1, 2, 3, 0, 0, 0];
        let mut v2 = vec![2, 5, 6];
        Solution::merge(&mut v1, 3, &mut v2, 3);
        assert_eq!(v1, vec![1, 2, 2, 3, 5, 6]);

        let mut v1 = vec![-1, 0, 0, 3, 3, 3, 0, 0, 0];
        let mut v2 = vec![1, 2, 2];
        Solution::merge(&mut v1, 6, &mut v2, 3);
        assert_eq!(v1, vec![-1, 0, 0, 1, 2, 2, 3, 3, 3]);
    }
}
