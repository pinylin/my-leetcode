/*
 * @lc app=leetcode id=434 lang=rust
 *
 * [434] Number of Segments in a String
 */
impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split(&[' '][..]).fold(0, |mut num, x| {
            if x != "" {
                num += 1
            };
            num
        })
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::count_segments("Hello, my name is John".to_owned()),
            5
        );
        assert_eq!(Solution::count_segments("     ".to_owned()), 0);
    }
}
