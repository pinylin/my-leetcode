/*
 * @lc app=leetcode id=443 lang=rust
 *
 * [443] String Compression
 */
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut loc = 0;
        let mut pre = chars[0];
        let mut count = 1;

        for i in 1..chars.len() {
            if chars[i] == pre {
                count += 1;
            } else {
                if count == 1 {
                    chars[loc] = pre;
                    loc += 1;
                } else {
                    chars[loc] = pre;
                    loc += 1;

                    let char_vec = Solution::num_to_char(count);
                    for x in &char_vec {
                        chars[loc] = *x;
                        loc += 1;
                    }
                }

                count = 1;
                pre = chars[i];
            }
        }

        if count == 1 {
            chars[loc] = pre;
            loc += 1;
        } else {
            chars[loc] = pre;
            loc += 1;

            let char_vec = Solution::num_to_char(count);
            for x in &char_vec {
                chars[loc] = *x;
                loc += 1;
            }
        }

        chars.truncate(loc);
        chars.len() as i32
    }
    fn num_to_char(mut n: i32) -> Vec<char> {
        let mut ret: Vec<char> = Vec::new();

        while n > 0 {
            let d = n % 10;
            ret.push((d as u8 + b'0') as char);
            n /= 10;
        }

        ret.reverse();
        ret
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        let mut chars = vec!['a', 'a', 'a', 'b', 'b', 'a', 'a'];
        assert_eq!(Solution::compress(&mut chars), 6);
        assert_eq!(chars, vec!['a', '3', 'b', '2', 'a', '2']);
    }
}
