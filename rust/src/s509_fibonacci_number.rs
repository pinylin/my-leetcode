/*
 * @lc app=leetcode id=509 lang=rust
 *
 * [509] Fibonacci Number
 */
use std::hash::Hash;
use std::collections::HashMap;
impl Solution {
    pub fn fib(n: i32) -> i32 {
        Solution::fib_memo(&mut HashMap::new(), n as u32) as i32
    }
    fn fib_memo (cache: &mut HashMap<u32, u32>, arg: u32) -> u32 {
        match arg {
            0 => 0,
            1 => 1,
            n => Solution::memoize(cache, Solution::fib_memo, n - 1) + 
                Solution::memoize(cache, Solution::fib_memo, arg - 2),
        }
    }
    fn memoize<A, R, F> (cache: &mut HashMap<A, R>, func: F, arg: A) -> R where
        A: Eq + Hash + Clone,
        R: Clone,
        F: Fn(&mut HashMap<A, R>, A) -> R
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

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::fib(10), 55);
        // assert_eq!(Solution::fib(40), 102334155);
    }
}
