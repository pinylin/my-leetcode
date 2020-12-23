/*
 * @lc app=leetcode.cn id=1154 lang=rust
 *
 * [1154] 一年中的第几天
 */

// @lc code=start
impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let ymd: Vec<i32> = date.split('-').map(|s| s.parse().unwrap_or(0)).collect();
        // 参考 s1360
        let is_leap = (ymd[0] % 4 == 0 && ymd[0] % 100 != 0) || ymd[0] % 400 == 0;

        (1..ymd[1])
            .map(|month| match month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                _ => {
                    if is_leap {
                        29
                    } else {
                        28
                    }
                }
            })
            .sum::<i32>()
            + ymd[2]
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::day_of_year("2003-03-01".to_owned()), 60);
        assert_eq!(Solution::day_of_year("2004-03-01".to_owned()), 61);
    }
}
