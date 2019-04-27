/*
 * @lc app=leetcode id=412 lang=rust
 *
 * [412] Fizz Buzz
 */
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res = Vec::with_capacity(n as usize);
        for i in 1..=n {
            if i % 3 == 0 {
                if i % 5 == 0 {
                    res.push("FizzBuzz".to_string());
                } else {
                    res.push("Fizz".to_string());
                }
            } else if i % 5 == 0 {
                res.push("Buzz".to_string());
            } else {
                res.push(format!("{}", i));
        }
    }
    res
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::fizz_buzz(15),
        vec![
            "1",
            "2",
            "Fizz",
            "4",
            "Buzz",
            "Fizz",
            "7",
            "8",
            "Fizz",
            "Buzz",
            "11",
            "Fizz",
            "13",
            "14",
            "FizzBuzz"
        ]
        );

    }
}
