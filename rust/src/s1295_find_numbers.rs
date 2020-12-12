/*
 * @lc app=leetcode.cn id=1295 lang=rust
 *
 * [1295] 统计位数为偶数的数字
 */

// @lc code=start
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold(0, |acc, x| acc + (x.to_string().len() as i32 + 1) % 2)

        // 方法2
        // nums.iter()
        //     .filter(|&&x| ((x as f32).log10() as i32 + 1) % 2 == 0)
        //     .count() as i32
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2)
    }
}
