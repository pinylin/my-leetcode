/*
 * @lc app=leetcode.cn id=1137 lang=rust
 *
 * [1137] 第 N 个泰波那契数
 */
// @lc code=start
use std::collections::HashMap;
use std::hash::Hash;
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        Solution::trib_memo(&mut HashMap::new(), n as u32) as i32
    }

    fn trib_memo(cache: &mut HashMap<u32, u32>, arg: u32) -> u32 {
        match arg {
            0 => 0,
            1 => 1,
            2 => 1,
            n => {
                Solution::memoize(cache, Solution::trib_memo, n - 1)
                    + Solution::memoize(cache, Solution::trib_memo, n - 2)
                    + Solution::memoize(cache, Solution::trib_memo, n - 3)
            }
        }
    }

    fn memoize<A, R, F>(cache: &mut HashMap<A, R>, func: F, arg: A) -> R
    where
        A: Eq + Hash + Clone,
        R: Clone,
        F: Fn(&mut HashMap<A, R>, A) -> R,
    {
        match cache.get(&arg).cloned() {
            Some(result) => result,
            None => {
                let result = func(cache, arg.clone());
                cache.insert(arg, result.clone());
                result
            }
        }
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
