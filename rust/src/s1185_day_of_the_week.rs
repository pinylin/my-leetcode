/*
 * @lc app=leetcode.cn id=1185 lang=rust
 *
 * [1185] 一周中的第几天
 */

// @lc code=start
impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let weekday_strs = vec![
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ];

        let mut days = day;
        for m in 1..month {
            match m {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => days += 31,
                2 => {
                    days += if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                        29
                    } else {
                        28
                    }
                }
                _ => days += 30,
            }
        }
        for y in 1..year {
            days += if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 {
                366
            } else {
                365
            };
        }
        weekday_strs[(days % 7) as usize].to_string()
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
            Solution::day_of_the_week(31, 8, 2019),
            "Saturday".to_owned()
        );
        assert_eq!(Solution::day_of_the_week(18, 7, 1999), "Sunday".to_owned());
    }
}
