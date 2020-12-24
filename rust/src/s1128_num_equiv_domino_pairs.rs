/*
 * @lc app=leetcode.cn id=1128 lang=rust
 *
 * [1128] 等价多米诺骨牌对的数量
 */

// @lc code=start
impl Solution {
    pub fn num_equiv_domino_pairs(mut dominoes: Vec<Vec<i32>>) -> i32 {
        let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for d in &mut dominoes {
            d.sort_unstable();
            *map.entry(d[0] * 10 + d[1]).or_default() += 1;
        }
        map.values().fold(0, |s, &v| s + v * (v - 1) / 2)
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
            Solution::num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]]),
            1
        );
    }
}
