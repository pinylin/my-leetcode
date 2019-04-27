/*
 * @lc app=leetcode id=409 lang=rust
 *
 * [409] Longest Palindrome
 */
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        use std::cmp;
        let mut band = vec![0i32; 58];
        for ch in s.bytes() {
            band[(ch - b'A') as usize] += 1;
        }
        let mut odds = 0;
        for item in band.iter() {
            odds += item & 1;
            // if item % 2 != 0 {
            //     odds += 1;
            // }
        }
        s.len() as i32 - cmp::max(0, odds - 1)
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::longest_palindrome("abccccdd".to_owned()), 7);

        assert_eq!(
            Solution::longest_palindrome("civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth".to_owned()),
            983
        );
    }
}
