/*
 * @lc app=leetcode.cn id=1200 lang=rust
 *
 * [1200] 最小绝对差
 */

// @lc code=start
impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        let mut min = i32::MAX;
        let mut res = Vec::new();
        for i in 1..arr.len() {
            let diff = arr[i] - arr[i - 1];
            if diff < min {
                min = diff;
                res.clear();
            } 
            if diff == min {
                res.push(vec![arr[i - 1], arr[i]]);
            }
        }
        res
    }
    pub fn minimum_abs_difference_2(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        let mut min = i32::MAX;
        let mut res = Vec::new();
        for i in 1..arr.len() {
            let diff = arr[i] - arr[i - 1];
            if diff < min {
                min = diff;
                res = vec![vec![arr[i - 1], arr[i]]];
            } else if diff == min {
                res.push(vec![arr[i - 1], arr[i]]);
            }
        }
        res
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::minimum_abs_difference(vec![4, 2, 1, 3]), vec![vec![1, 2], vec![2, 3], vec![3, 4]]);
        assert_eq!(Solution::minimum_abs_difference(vec![1, 3, 6, 10, 15]), vec![vec![1, 3]]);
        assert_eq!(Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]), vec![vec![-14, -10], vec![19, 23], vec![23, 27]]);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;

    #[bench]
    fn vec_clear(b: &mut Bencher) {
        b.iter(|| Solution::minimum_abs_difference(vec![4, 2, 1, 3]));
        b.iter(|| Solution::minimum_abs_difference(vec![1, 3, 6, 10, 15]));
        b.iter(|| Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]));
    }

    #[bench]
    fn new_vec (b: &mut Bencher) {
        b.iter(|| Solution::minimum_abs_difference_2(vec![4, 2, 1, 3]));
        b.iter(|| Solution::minimum_abs_difference_2(vec![1, 3, 6, 10, 15]));
        b.iter(|| Solution::minimum_abs_difference_2(vec![3, 8, -10, 23, 19, -4, -14, 27]));
    }
}
