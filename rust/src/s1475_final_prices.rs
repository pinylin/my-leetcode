/*
 * @lc app=leetcode.cn id=1475 lang=rust
 *
 * [1475] 商品折扣后的最终价格
 */

// @lc code=start
impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; prices.len()];
        (0..prices.len()).for_each(|i| {
            res[i] = prices[i]
                - ((i + 1)..prices.len())
                    .find(|&j| prices[j] <= prices[i])
                    .map_or(0, |j| prices[j]);
        });
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
        assert_eq!(Solution::final_prices(vec![10, 1, 1, 6]), vec![9, 0, 1, 6])
    }
}
