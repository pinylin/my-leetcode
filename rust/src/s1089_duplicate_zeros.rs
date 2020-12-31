/*
 * @lc app=leetcode.cn id=1089 lang=rust
 *
 * [1089] 复写零
 */

// @lc code=start
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let l = arr.len();
        let mut i = 0;
        while i < l {
            if arr[i] == 0 {
                arr.insert(i + 1, 0);
                i += 2;
            } else {
                i += 1;
            }
        }

        arr.drain(l..);
    }

    pub fn duplicate_zeros_pop(arr: &mut Vec<i32>) {
        let l = arr.len();
        let mut i = 0;
        while i < l {
            if arr[i] == 0 {
                arr.insert(i + 1, 0);
                i += 2;
            } else {
                i += 1;
            }
        }
        while arr.len() != l {
            arr.pop();
        }
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;

    #[bench]
    fn drain(b: &mut Bencher) {
        let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
        b.iter(|| Solution::duplicate_zeros(arr.as_mut()));
    }

    #[bench]
    fn pop(b: &mut Bencher) {
        let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
        b.iter(|| Solution::duplicate_zeros_pop(arr.as_mut()));
    }
}
