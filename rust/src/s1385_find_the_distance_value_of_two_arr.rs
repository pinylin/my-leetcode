/*
 * @lc app=leetcode.cn id=1385 lang=rust
 *
 * [1385] 两个数组间的距离值
 */

// @lc code=start
impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        // 方法1
        // 迭代数组arr1，迭代数组arr2，如果不存在|arr1[i] - arr2[j]| <= d，计数加1
        // 返回计数值
        // O(n^2)
        // Passed 0ms 2.1mb
        // arr1.iter().filter(|&&n1| arr2.iter().all(|&n2| (n1 - n2).abs() > d)).count() as i32

        // 方法2
        // 排序arr2，然后二分查找最接近或者相等于arr1[i]的值，如果这个值距离大于d，则计数加1
        // 返回计数值
        // O(nlogn)
        // Passed 0ms 2.1mb
        let mut arr2 = arr2;
        arr2.sort_unstable();
        arr1.iter()
            .filter(|&n1| {
                let j = arr2.binary_search(n1).unwrap_or_else(|x| x);
                let j = std::cmp::min(j, arr2.len() - 1); //  可能等于arr2.len()
                std::cmp::min(
                    (*n1 - arr2[j.saturating_sub(1)]).abs(),
                    (*n1 - arr2[j]).abs(),
                ) > d
            })
            .count() as i32
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
            Solution::find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2),
            2
        );
        assert_eq!(
            Solution::find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3),
            2
        );
        assert_eq!(
            Solution::find_the_distance_value(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6),
            1
        );
    }
}
