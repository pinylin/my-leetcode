/*
 * @lc app=leetcode id=438 lang=rust
 *
 * [438] Find All Anagrams in a String
 */
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.is_empty() {return vec![]}
        let mut band = vec![0i16; 256];
        let mut res = Vec::new();
        for ch in p.bytes() {
            band[ch as usize] += 1;
        }
        let mut left = 0usize;
        let mut right = 0usize;
        let mut cnt = p.len() as i16;
        let sv = s.as_bytes();
        while right < sv.len() {
            if band[sv[right] as usize] > 0 {               
                cnt -= 1;   
            }
            band[sv[right] as usize] -= 1;  
            right += 1;
            if cnt == 0 { res.push(left as i32); }
            if right - left == p.len()  {
                if band[sv[left] as usize] >= 0 {
                    cnt += 1;
                }
                band[sv[left] as usize] += 1;
                left += 1;
                
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
        assert_eq!(
            Solution::find_anagrams("cbaebabacd".to_owned(), "abc".to_owned()),
            vec![0, 6]
        );
        assert_eq!(
            Solution::find_anagrams("abab".to_owned(), "ab".to_owned()), 
            vec![0, 1, 2]
        );
    }
}
