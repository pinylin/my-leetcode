/*
 * @lc app=leetcode.cn id=1399 lang=rust
 *
 * [1399] 统计最大组的数目
 */

// @lc code=start
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut arr = vec![0; 37];
        (1..=n).for_each(|mut x| {
            // let sum: u32 = x.to_string().chars().map(|ch| ch.to_digit(10).unwrap_or(0)).sum();
            let mut sum = 0;
            while x > 0 {
                sum += x % 10;
                x /= 10;
            }
            arr[sum as usize] += 1;
        });
        let max = *arr.iter().max().unwrap();
        arr.iter().filter(|&&n| n == max).count() as i32
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::count_largest_group(15), 6);
        assert_eq!(Solution::count_largest_group(24), 5);
    }
}
