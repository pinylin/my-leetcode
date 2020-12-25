/*
 * @lc app=leetcode.cn id=39 lang=rust
 *
 * [39] 组合总和
 */

// @lc code=start
impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        Solution::find_target(&candidates, target)
            .into_iter()
            .map(|mut v| {
                v.sort();
                v
            })
            .collect()
    }

    pub fn find_target(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut all = vec![];
        for (i, &c) in candidates.iter().take_while(|c| **c <= target).enumerate() {
            if c == target {
                all.push(vec![c]);
                continue;
            }
            let partial = Solution::find_target(&candidates[i..], target - c);
            for v in partial.into_iter() {
                let mut v = v.clone();
                v.push(c);
                all.push(v);
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
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
    }
}
