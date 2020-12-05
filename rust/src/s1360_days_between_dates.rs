/*
 * @lc app=leetcode.cn id=1360 lang=rust
 *
 * [1360] 日期之间隔几天
 */

// @lc code=start
impl Solution {
    // refer: https://github.com/ruislan/leetcode/blob/df09787718d007576cad6614101726c81d20f9c2/src/q/q1360.rs
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let ymd1: Vec<i32> = date1.split('-').map(|s| s.parse().unwrap_or(0)).collect();
        let ymd2: Vec<i32> = date2.split('-').map(|s| s.parse().unwrap_or(0)).collect();
        (Self::days_from_1971(ymd1) - Self::days_from_1971(ymd2)).abs()
    }

    fn days_from_1971(ymd: Vec<i32>) -> i32 {
        (1971..=ymd[0])
            .map(|year| {
                let is_leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
                if year != ymd[0] {
                    if is_leap {
                        366
                    } else {
                        365
                    }
                } else {
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
            })
            .sum()
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
            Solution::days_between_dates("1971-06-29".to_string(), "2010-09-23".to_string()),
            14331
        );
        assert_eq!(
            Solution::days_between_dates("2019-06-29".to_string(), "2019-06-30".to_string()),
            1
        );
        assert_eq!(
            Solution::days_between_dates("2019-06-30".to_string(), "2019-06-29".to_string()),
            1
        );
        assert_eq!(
            Solution::days_between_dates("2019-06-29".to_string(), "2019-06-29".to_string()),
            0
        );
        assert_eq!(
            Solution::days_between_dates("2020-01-15".to_string(), "2019-12-31".to_string()),
            15
        );
        assert_eq!(
            Solution::days_between_dates("2000-03-30".to_string(), "2019-03-30".to_string()),
            6939
        );
        assert_eq!(
            Solution::days_between_dates("1971-02-01".to_string(), "2020-06-28".to_string()),
            18045
        );
    }
}
