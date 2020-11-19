/*
 * @lc app=leetcode.cn id=1534 lang=rust
 *
 * [1534] 统计好三元组
 */

// @lc code=start
impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let length: usize = arr.len();
        let mut count = 0;
        for i in 0..length - 2 {
            for j in i + 1..length - 1 {
                if (arr[i] - arr[j]).abs() > a {
                    continue;
                }
                for k in j + 1..length {
                    if (arr[j] - arr[k]).abs() > b {
                        continue;
                    }
                    if (arr[i] - arr[k]).abs() > c {
                        continue;
                    }
                    count += 1;
                }
            }
        }
        count
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            4,
            Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3)
        );
        assert_eq!(
            0,
            Solution::count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1)
        );
        assert_eq!(
            12,
            Solution::count_good_triplets(vec![7, 3, 7, 3, 12, 1, 12, 2, 3], 5, 8, 1)
        );
    }
}
