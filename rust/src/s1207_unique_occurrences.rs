/*
 * @lc app=leetcode.cn id=1207 lang=rust
 *
 * [1207] 独一无二的出现次数
 */
use std::collections::{HashMap, HashSet};
// @lc code=start
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        // let mut map = HashMap::new();
        // let mut set = HashSet::new();
        // for num in arr {
        //     *map.entry(num).or_insert(0) += 1;
        // }
        // for (_, v) in map {
        //     if !set.insert(v) {
        //         return false;
        //     }
        // }
        // true

        // 更优雅的写法
        let map: HashMap<i32, usize> = arr.iter().fold(HashMap::new(), |mut acc, x| {
            *acc.entry(*x).or_insert(0) += 1;
            acc
        });
        let set: HashSet<&usize> = map.values().collect();
        map.len() == set.len()
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
        assert_eq!(Solution::unique_occurrences(vec![1, 2]), false);
        assert_eq!(
            Solution::unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]),
            true
        );
    }
}
