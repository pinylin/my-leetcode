/*
 * @lc app=leetcode id=405 lang=rust
 *
 * [405] Convert a Number to Hexadecimal
 */
impl Solution {
    pub fn to_hex(num: i32) -> String {
        let bits = vec![
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];
        let mut num = num as u32;
        if num == 0 {
            return "0".to_owned();
        }
        let mut ret = String::new();
        while num > 0 {
            ret.push(bits[(num % 16) as usize]);
            num /= 16;
        }
        ret.chars().rev().collect()
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::to_hex(26), "1a".to_owned());
        assert_eq!(Solution::to_hex(-1), "ffffffff".to_owned());
    }
}
