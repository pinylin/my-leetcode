/*
 * @lc app=leetcode.cn id=1281 lang=rust
 *
 * [1281] 整数的各位积和之差
 */

// @lc code=start
impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut decomposed: Vec<i32> = Vec::new();

        let mut cur = n;
        while cur != 0 {
            decomposed.push(cur % 10);
            cur /= 10;
        }

        return decomposed.iter().product::<i32>() - decomposed.iter().sum::<i32>();
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::subtract_product_and_sum(234), 15);
        assert_eq!(Solution::subtract_product_and_sum(4421), 21);
    }
}
