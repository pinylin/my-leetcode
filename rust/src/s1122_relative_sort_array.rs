/*
 * @lc app=leetcode.cn id=1122 lang=rust
 *
 * [1122] 数组的相对排序
 */

// @lc code=start
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        let mut res = Vec::new();
        for num in arr1 {
            *map.entry(num).or_default() += 1;
        }
        for num in arr2 {
            let count = *map.get(&(num)).unwrap_or(&0);
            res.extend(vec![num; count as usize]);
            *map.get_mut(&num).unwrap() = 0;
        }
        let mut appended = Vec::new();
        for (k, v) in map {
            if v != 0 {
                appended.push((k, v))
            }
        }
        appended.sort_unstable();
        for (k, v) in appended {
            res.extend(vec![k; v as usize]);
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
        assert_eq!(
            Solution::relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6]
            ),
            vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
        );
    }
}
