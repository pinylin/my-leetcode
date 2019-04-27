/*
 * @lc app=leetcode id=367 lang=rust
 *
 * [367] Valid Perfect Square
 */
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num < 2 {
            return true;
        }
        let num = i64::from(num);
        let (mut left, mut right) = (2, num / 2);
        while left <= right {
            let mid = (left + right) / 2;
            let msq = mid * mid;
            if msq == num {
                return true;
            } else if msq < num {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        false
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::is_perfect_square(25), true);
        assert_eq!(Solution::is_perfect_square(808201), true);
    }
}
