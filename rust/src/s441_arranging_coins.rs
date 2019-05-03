/*
 * @lc app=leetcode id=441 lang=rust
 *
 * [441] Arranging Coins
 */
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        (-1 + (1.0 + 8.0 * (n as f64)).sqrt() as i32) / 2
        // if n < 2 { return n;}
        // let n = n as i64;
        // for i in 1..n+1 {
        //     let temp = i * (i + 1) - n * 2;
        //     if temp > 0 {
        //         return i as i32 - 1;
        //     } else if temp == 0 {
        //         return i as i32;
        //     }
        // }
        // 0
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::arrange_coins(2), 1);
        assert_eq!(Solution::arrange_coins(1804289383), 60070);
    }
}
