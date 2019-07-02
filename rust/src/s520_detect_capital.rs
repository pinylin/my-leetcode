/*
 * @lc app=leetcode id=520 lang=rust
 *
 * [520] Detect Capital
 */
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let z_up = b'Z';
        let count = word.bytes().filter(|s| z_up >= *s).count();
        count == 0 || count == word.len() || (count == 1 && z_up >= word.into_bytes()[0])
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::detect_capital_use("FlaG".to_owned()), false);
        assert_eq!(Solution::detect_capital_use("Abc".to_owned()), true);
    }
}
