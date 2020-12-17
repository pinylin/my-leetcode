/*
 * @lc app=leetcode.cn id=1175 lang=rust
 *
 * [1175] 质数排列
 */

// @lc code=start
impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let m = 1_000_000_007;
        let primes = Solution::count_primes(n + 1);

        let mut res = 1_u64;
        let composite = n as u64 - primes;
        for i in 2..=composite {
            res = (res * i) % m;
        }
        for i in 2..=primes {
            res = (res * i) % m;
        }

        res as i32
    }

    // s204 小于n的素数的
    pub fn count_primes(n: i32) -> u64 {
        if n < 3 {
            return 0;
        }
        let mut nums = vec![true; n as usize];
        for i in 2..=(f64::from(n).sqrt() as usize) {
            if nums[i] {
                for j in (i..n as usize).take_while(|x| x * i < n as usize) {
                    nums[i * j] = false;
                }
            }
        }
        nums.into_iter().filter(|&x| x).count() as u64 - 2
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::num_prime_arrangements(2), 1);
        assert_eq!(Solution::num_prime_arrangements(5), 12);
        assert_eq!(Solution::num_prime_arrangements(100), 682289015);
    }
}
