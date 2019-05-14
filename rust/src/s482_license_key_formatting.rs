/*
 * @lc app=leetcode id=482 lang=rust
 *
 * [482] License Key Formatting
 */
impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut count = 0;
        let mut res = vec![];
        for item in s.chars().rev() {
            if count == k {
                res.push('-');
                count = 0;
            }
            if item != '-' && count < k {
                res.push(item);
                count += 1;
            }
        }
        if res.last() == Some(&'-') {
            res.remove(res.len() - 1);
        }
        res.iter().rev().collect::<String>().to_ascii_uppercase()
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::license_key_formatting("--a-a--aa--".to_owned(), 2),
            "AA-AA".to_owned()
        );
    }
}
