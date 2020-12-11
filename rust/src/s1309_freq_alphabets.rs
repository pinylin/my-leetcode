/*
 * @lc app=leetcode.cn id=1309 lang=rust
 *
 * [1309] 解码字母到整数映射
 */

// @lc code=start
impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut res = String::new();
        let mut i: usize = 0;
        while i < s.len() {
            if i + 2 < s.len() && s.get(i + 2..i + 3).unwrap() == "#" {
                res.push((s.get(i..i + 2).unwrap().parse::<u8>().unwrap() + 96) as char);
                i += 3;
            } else {
                res.push((s.get(i..i + 1).unwrap().parse::<u8>().unwrap() + 96) as char);
                i += 1;
            }
        }
        res
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
            &Solution::freq_alphabets(
                "12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#".to_string()
            ),
            "abcdefghijklmnopqrstuvwxyz"
        );
        assert_eq!(&Solution::freq_alphabets("25#".to_string()), "y");
        assert_eq!(&Solution::freq_alphabets("1326#".to_string()), "acz");
        assert_eq!(&Solution::freq_alphabets("10#11#12".to_string()), "jkab");
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;

    #[bench]
    fn freq_alphabets(b: &mut Bencher) {
        b.iter(|| {
            Solution::freq_alphabets(
                "12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#".to_string(),
            )
        });
        b.iter(|| Solution::freq_alphabets("25#".to_string()));
        b.iter(|| Solution::freq_alphabets("1326#".to_string()));
        b.iter(|| Solution::freq_alphabets("10#11#12".to_string()));
    }
}
