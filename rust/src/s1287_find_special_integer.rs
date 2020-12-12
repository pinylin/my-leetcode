/*
 * @lc app=leetcode.cn id=1287 lang=rust
 *
 * [1287] 有序数组中出现次数超过25%的元素
 */

// @lc code=start
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        // 注意到数组有序
        let step = arr.len() / 4;
        for i in 0..arr.len() - 1 {
            if arr[i] == arr[i + step] {
                return arr[i];
            }
        }
        arr[0]
    }

    pub fn find_special_integer_2(arr: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut count = 0;
        let mut map = std::collections::HashMap::new();
        for num in arr.iter() {
            *map.entry(*num).or_insert(0) += 1;
        }
        for (k, v) in map {
            if v > count {
                count = std::cmp::max(count, v);
                res = k;
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
        assert_eq!(Solution::find_special_integer(vec![7, 7, 7, 7, 7, 7, 7]), 7);
        assert_eq!(
            Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]),
            6
        );
        assert_eq!(
            Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]),
            6
        );
    }
}
