/*
 * @lc app=leetcode id=401 lang=rust
 *
 * [401] Binary Watch
 */
impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        fn bit_count(num: i32) -> i32 {
            format!("{:b}", num).chars().filter(|c| *c == '1').count() as i32
        }

        for h in 0..12 {
            for m in 0..60 {
                // * 64 只是为了移位，所以你也可以 * 128
                if bit_count(h * 64 + m) == num {
                    result.push(format!("{}:{:02}", h, m));
                }
            }
        }

        result
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::read_binary_watch(0), vec!["0:00"]);
        assert_eq!(
            Solution::read_binary_watch(1),
            vec!["0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"]
        );
    }
}
