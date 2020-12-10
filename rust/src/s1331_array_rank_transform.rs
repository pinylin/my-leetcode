/*
 * @lc app=leetcode.cn id=1331 lang=rust
 *
 * [1331] 数组序号转换
 */

// @lc code=start

use std::collections::{HashMap, HashSet};
// use std::iter::FromIterator;
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let set: HashSet<i32> = arr.clone().iter().map(|x| *x).collect();
        // let set: HashSet<i32> = HashSet::from_iter(arr.clone().into_iter());
        let mut arr2: Vec<i32> = set.into_iter().collect();
        // let mut arr2 = Vec::from_iter(set);
        arr2.sort();

        let map: HashMap<i32, i32> = arr2
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i as i32 + 1))
            .collect();

        arr.into_iter().map(|x| *map.get(&x).unwrap()).collect()
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
            Solution::array_rank_transform(vec![100, 100, 100]),
            vec![1, 1, 1]
        );
        assert_eq!(
            Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
        );
    }
}
