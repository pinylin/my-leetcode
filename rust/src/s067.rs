/*
 * @lc app=leetcode id=67 lang=rust
 *
 * [67] Add Binary
 *
 * https://leetcode.com/problems/add-binary/description/
 *
 * algorithms
 * Easy (38.03%)
 * Total Accepted:    281.6K
 * Total Submissions: 737.8K
 * Testcase Example:  '"11"\n"1"'
 *
 * Given two binary strings, return their sum (also a binary string).
 * 
 * The input strings are both non-empty and contains only characters 1 or 0.
 * 
 * Example 1:
 * 
 * 
 * Input: a = "11", b = "1"
 * Output: "100"
 * 
 * Example 2:
 * 
 * 
 * Input: a = "1010", b = "1011"
 * Output: "10101"
 * 
 */
use std::char::from_digit;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut buf = Vec::with_capacity(usize::max(a.len(), b.len())    + 1);
        let mut a: Vec<char> = a.chars().collect();
        let mut b: Vec<char> = b.chars().collect();
        let mut carry = 0;
        while !(a.is_empty() && b.is_empty()) {
            let mut sum = a.pop().map_or(0, |ch| ch.to_digit(10).unwrap())
                + b.pop().map_or(0, |ch| ch.to_digit(10).unwrap()) + carry;
            if sum > 1 {
                sum -= 2;
                carry = 1;
            } else {
                carry = 0;
            }
            buf.push(from_digit(sum, 10).unwrap())
        }
        if carry > 0 {
            buf.push('1')
        }
        buf.into_iter().rev().collect()
    }

    pub fn add_binary_2(a: String, b: String) -> String {
        let (mut b1, mut b2) = (a.bytes().rev(), b.bytes().rev());
        let mut ret = vec![];
        let mut carry = 0;
        for _ in 0..a.len().max(b.len()) {
            let mut bit = b1.next().unwrap_or(b'0') + b2.next().unwrap_or(b'0') - b'0' * 2 + carry;
            carry = 0;
            if bit >= 2 {
                bit -= 2;
                carry = 1;
            }
            ret.push((bit + b'0') as char);
        }
        if carry == 1 {
            ret.push('1');
        }
        ret.reverse();
        ret.iter().collect::<String>()
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::add_binary("11".to_owned(), "1".to_owned()), "100");
        assert_eq!(
            Solution::add_binary("1010".to_owned(), "1011".to_owned()),
            "10101"
        );
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use super::Solution;
    use self::test::Bencher;
    #[bench]
    fn add_binary(b: &mut Bencher) {
        b.iter(|| Solution::add_binary("10".to_owned(), "1".to_owned()))
    }
    
    #[bench]
    fn add_binary_2(b: &mut Bencher) {
        b.iter(|| Solution::add_binary("10".to_owned(), "1".to_owned()))
    }
}