/*
 * @lc app=leetcode.cn id=1046 lang=rust
 *
 * [1046] 最后一块石头的重量
 */

// @lc code=start
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = std::collections::BinaryHeap::from(stones);

        loop {
            if let Some(y) = heap.pop() {
                if let Some(x) = heap.pop() {
                    if y > x {
                        heap.push(y - x);
                    }
                } else {
                    return y;
                }
            } else {
                return 0;
            }
        }
    }
}
// @lc code=end

pub struct Solution {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }
}
