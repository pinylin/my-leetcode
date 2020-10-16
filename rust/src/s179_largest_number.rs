/*
 * @lc app=leetcode id=179 lang=rust
 *
 * [179] Largest Number
 *
 * https://leetcode.com/problems/largest-number/description/
 *
 * algorithms
 * Medium (28.42%)
 * Likes:    2188
 * Dislikes: 255
 * Total Accepted:    196.4K
 * Total Submissions: 674K
 * Testcase Example:  '[10,2]'
 *
 * Given a list of non negative integers, arrange them such that they form the
 * largest number.
 *
 * Example 1:
 *
 *
 * Input: [10,2]
 * Output: "210"
 *
 * Example 2:
 *
 *
 * Input: [3,30,34,5,9]
 * Output: "9534330"
 *
 *
 * Note: The result may be very large, so you need to return a string instead
 * of an integer.
 *
 */

// @lc code=start

use std::cmp::Ordering;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums;
        nums.sort_unstable_by(|&a, &b| {
            let ab = format!("{}{}", a, b);
            let ba = format!("{}{}", b, a);
            ab.cmp(&ba)
        });
        if nums.last().unwrap() == &0 {
            return "0".to_owned();
        }
        nums.into_iter().rev().fold(String::new(), |mut acc, n| {
            acc.push_str(&n.to_string());
            acc
        })
    }

    #[inline]
    fn wow_cmp(a: &str, b: &str) -> Ordering {
        let iter1 = a.bytes().chain(b.bytes());
        let iter2 = b.bytes().chain(a.bytes());
        for (i, j) in iter1.zip(iter2) {
            match i.cmp(&j) {
                Ordering::Equal => continue,
                cmp => return cmp,
            }
        }
        Ordering::Equal
    }

    pub fn largest_number_u8(nums: Vec<i32>) -> String {
        let mut nums = nums.iter().map(|n| n.to_string()).collect::<Vec<_>>();
        nums.sort_unstable_by(|a, b| Solution::wow_cmp(b, a));
        match nums.join("").trim_start_matches('0') {
            "" => "0".to_owned(),
            s => s.to_owned(),
        }
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::cmp::Ordering;

    #[test]
    fn test_wow_cmp() {
        assert_eq!(Solution::wow_cmp("3", "30"), Ordering::Greater);
        assert_eq!(Solution::wow_cmp("3", "34"), Ordering::Less);
        assert_eq!(Solution::wow_cmp("34", "30"), Ordering::Greater);
        assert_eq!(Solution::wow_cmp("10", "2"), Ordering::Less);
        assert_eq!(Solution::wow_cmp("12", "128"), Ordering::Less);
        assert_eq!(Solution::wow_cmp("128", "12"), Ordering::Greater);
        assert_eq!(Solution::wow_cmp("121", "12"), Ordering::Less);
    }

    #[test]
    fn test() {
        assert_eq!(Solution::largest_number(vec![10, 2]), "210");
        assert_eq!(Solution::largest_number(vec![3, 30, 34, 5, 9]), "9534330");
        assert_eq!(Solution::largest_number(vec![128, 12]), "12812");
        assert_eq!(Solution::largest_number(vec![0, 10]), "100");
        assert_eq!(Solution::largest_number(vec![0, 0]), "0");
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;

    #[bench]
    fn sort_by_string(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(Solution::largest_number(vec![10, 2]), "210");
            assert_eq!(Solution::largest_number(vec![3, 30, 34, 5, 9]), "9534330");
            assert_eq!(Solution::largest_number(vec![128, 12]), "12812");
            assert_eq!(Solution::largest_number(vec![0, 10]), "100");
            assert_eq!(Solution::largest_number(vec![0, 0]), "0");
        });
    }

    #[bench]
    fn sort_by_byte(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(Solution::largest_number_u8(vec![10, 2]), "210");
            assert_eq!(
                Solution::largest_number_u8(vec![3, 30, 34, 5, 9]),
                "9534330"
            );
            assert_eq!(Solution::largest_number_u8(vec![128, 12]), "12812");
            assert_eq!(Solution::largest_number_u8(vec![0, 10]), "100");
            assert_eq!(Solution::largest_number_u8(vec![0, 0]), "0");
        });
    }
}
