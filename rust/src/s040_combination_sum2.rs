/*
 * @lc app=leetcode.cn id=40 lang=rust
 *
 * [40] 组合总和 II
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();

        let mut res = Vec::new();
        let mut path = Vec::new();
        Self::dfs(&candidates, target, 0, &mut res, &mut path);
        res
    }

    fn dfs(
        candidates: &[i32],
        target: i32,
        begin: usize,
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
    ) {
        if target < 0 {
            // 当前分支不符合
            return;
        }
        if target == 0 {
            // 当前分支符合，加入结果集
            result.push(path.to_vec());
            return;
        }
        for i in begin..candidates.len() {
            if i > begin && candidates[i] == candidates[i - 1] {
                // 忽略重复的分支
                continue;
            }
            let new_target = target - candidates[i];
            if new_target < 0 {
                return;
            }
            path.push(candidates[i]);
            Self::dfs(candidates, new_target, i + 1, result, path);
            path.pop();
        }
    }

    pub fn combination_sum2_recursion(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        Solution::find_target(&candidates, target)
            .into_iter()
            .map(|mut v| {
                v.sort();
                v
            })
            .collect()
    }

    pub fn find_target(candidates: &[i32], target: i32) -> HashSet<Vec<i32>> {
        let mut all = HashSet::new();
        for (i, &c) in candidates.iter().take_while(|c| **c <= target).enumerate() {
            if c == target {
                all.insert(vec![c]);
                continue;
            }
            let partial = Self::find_target(&candidates[i + 1..], target - c);
            for v in partial.into_iter() {
                let mut v = v.clone();
                v.push(c);
                all.insert(v);
            }
        }
        all
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
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5]]
        );
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;

    #[bench]
    fn dfs(b: &mut Bencher) {
        b.iter(|| Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8));
        b.iter(|| Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5));
    }

    #[bench]
    fn recursion(b: &mut Bencher) {
        b.iter(|| Solution::combination_sum2_recursion(vec![10, 1, 2, 7, 6, 1, 5], 8));
        b.iter(|| Solution::combination_sum2_recursion(vec![2, 5, 2, 1, 2], 5));
    }
}
