/*
 * @lc app=leetcode.cn id=1417 lang=rust
 *
 * [1417] 重新格式化字符串
 */

// @lc code=start
impl Solution {
    pub fn reformat(s: String) -> String {
        let (mut digits, mut letters): (Vec<u8>, Vec<u8>) =
            s.bytes().partition(|ch| ch.is_ascii_digit());
        let mut ret = vec![];

        let mut iterator = if digits.len() == letters.len() + 1 {
            ret.push(digits.pop().unwrap());
            letters.iter().zip(digits.iter())
        } else if digits.len() + 1 == letters.len() {
            ret.push(letters.pop().unwrap());
            digits.iter().zip(letters.iter())
        } else if digits.len() == letters.len() {
            digits.iter().zip(letters.iter())
        } else {
            [].iter().zip([].iter())
        };

        while let Some((&ch0, &ch1)) = iterator.next() {
            ret.push(ch0);
            ret.push(ch1);
        }

        String::from_utf8(ret).unwrap()
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
            Solution::reformat("covid2019".to_owned()),
            "d2c0o1v9i".to_owned()
        );
        assert_eq!(Solution::reformat("ab123".to_owned()), "3a1b2".to_owned())
    }
}
