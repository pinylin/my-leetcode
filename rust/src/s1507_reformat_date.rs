/*
 * @lc app=leetcode.cn id=1507 lang=rust
 *
 * [1507] 转变日期格式
 */

// @lc code=start
impl Solution {
    pub fn reformat_date(date: String) -> String {
        let mut map = std::collections::HashMap::new();
        [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ]
        .iter()
        .enumerate()
        .for_each(|(i, &month)| {
            map.insert(month, i + 1);
        });
        let ymd: Vec<&str> = date.split_whitespace().collect();
        format!(
            "{:04}-{:02}-{:02}",
            ymd[2].parse::<usize>().unwrap_or(0),
            map[ymd[1]],
            ymd[0]
                .chars()
                .take_while(|&ch| ch.is_ascii_digit())
                .collect::<String>()
                .parse::<usize>()
                .unwrap_or(0)
        )
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
            Solution::reformat_date("20th Oct 2052".to_owned()),
            "2052-10-20".to_owned()
        );
    }
}
