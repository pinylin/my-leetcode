/*
 * @lc app=leetcode id=500 lang=rust
 *
 * [500] Keyboard Row
 */
impl Solution {
    // refer:  https://github.com/sunwayforever/leetcode/blob/4d634e1099c5498d0a767b7c0f87de24f7b8a43a/rust/keyboard_row/src/main.rs
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut mapping: HashMap<char, i32> = HashMap::new();
        "qwertyuiop".chars().for_each(|c| {
            mapping.insert(c, 1);
        });
        "asdfghjkl".chars().for_each(|c| {
            mapping.insert(c, 2);
        });
        "zxcvbnm".chars().for_each(|c| {
            mapping.insert(c, 3);
        });
        words
            .iter()
            .filter(|w| {
                let w = w.to_lowercase();
                let id = *mapping.get(&w.chars().next().unwrap()).unwrap();
                w.chars().all(|c| mapping[&c] == id)
            })
            .cloned()
            .collect()
    }
}

