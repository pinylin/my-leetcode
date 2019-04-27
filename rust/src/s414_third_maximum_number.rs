/*
 * @lc app=leetcode id=414 lang=rust
 *
 * [414] Third Maximum Number
 */
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let (mut first, mut second, mut third) = (std::i32::MIN, std::i32::MIN, std::i32::MIN);
        for num in nums.iter() {
            if num > &first {
                third = second;
                second = first;
                first = *num;
            } else if num > &second && num < &first {
                third = second;
                second = *num;
            } else if num > &third && num < &second {
                third = *num;
            }
        }
        if third == std::i32::MIN {
            let mut set = HashSet::new();
            for num in nums.iter() {
                set.insert(num);
            }
            if set.len() == 3 {
                return third;
            }
        }
        if third == std::i32::MIN || third == second {
            return first;
        }
        third
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::third_max(vec![1, 1, 2]), 2);
        assert_eq!(Solution::third_max(vec![1, 2, -3]), -3);
        assert_eq!(Solution::third_max(vec![1, 2, -2147483648]), -2147483648);
        assert_eq!(Solution::third_max(vec![1, -2147483648, 2]), -2147483648);
        assert_eq!(Solution::third_max(vec![-2147483648, 2, 2]), 2);
    }
}
