/*
 * @lc app=leetcode id=415 lang=rust
 *
 * [415] Add Strings
 */
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let (mut num1, mut num2) = (num1.bytes().rev(), num2.bytes().rev());
        let mut ret = vec![];
        let mut carry = 0;
        loop {
            match (num1.next(), num2.next()) {
                (None, None) => break,
                (a, b) => {
                    let mut n = a.unwrap_or(b'0') + b.unwrap_or(b'0') - b'0' + carry;
                    carry = 0;
                    if n > b'9' {
                        n -= 10;
                        carry = 1;
                    }
                    ret.push(n as char);
                }
            }
        }
        if carry != 0 {
            ret.push('1');
        }
        ret.iter().rev().collect()
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::add_strings("0".to_owned(), "0".to_owned()),
            "0".to_owned()
        );
        assert_eq!(
            Solution::add_strings("111".to_owned(), "999".to_owned()),
            "1110".to_owned()
        );
        assert_eq!(
            Solution::add_strings("123".to_owned(), "456".to_owned()),
            "579".to_owned()
        );
    }
}
