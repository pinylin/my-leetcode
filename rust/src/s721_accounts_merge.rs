/*
 * @lc app=leetcode.cn id=721 lang=rust
 *
 * [721] 账户合并
 */

// @lc code=start
use std::collections::{HashMap, HashSet};
impl Solution {
    // 方法1 并查集
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // init ufs
        let mut ufs = UnionFindSet::new();
        for acc in &accounts {
            for (mail1, mail2) in acc.iter().skip(1).zip(acc.iter().skip(2)) {
                ufs.union(&mail1[..], &mail2[..]);
            }
        }
        // create map group => mail list
        let mut group: HashMap<&str, Vec<&str>> = HashMap::new();
        for acc in &accounts {
            let mut acc_iter = acc.into_iter();
            let name = acc_iter.next().unwrap();
            for mail in acc_iter {
                let group_mail = ufs.find(&mail);
                let entry = group.entry(group_mail).or_insert_with(|| vec![name]);

                if let Err(i) = (*entry)[1..].binary_search(&&mail[..]) {
                    (*entry).insert(i + 1, mail);
                }
            }
        }
        // base on owner, group => mail list  to owner=> mail list
        group
            .into_iter()
            .map(|(_, v)| v.into_iter().map(|v| v.to_string()).collect::<Vec<_>>())
            .collect()
    }
    // 方法2 先建立连通图graph，然后深度优先搜索
    pub fn accounts_merge_dfs(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut graph = HashMap::new();
        let mut mail2account = HashMap::new();
        for i in 0..accounts.len() {
            let name = &accounts[i][0];
            for j in 1..accounts[i].len() {
                mail2account.entry(&accounts[i][j]).or_insert(name);
            }

            for j in 2..accounts[i].len() {
                graph
                    .entry(&accounts[i][j - 1])
                    .or_insert(Vec::new())
                    .push(&accounts[i][j]);
                graph
                    .entry(&accounts[i][j])
                    .or_insert(Vec::new())
                    .push(&accounts[i][j - 1]);
            }
        }

        let mut visited = HashSet::new();
        let mut res = Vec::new();
        for i in 0..accounts.len() {
            if accounts[i].len() > 0 && !visited.contains(&accounts[i][1]) {
                let mut stack = vec![&accounts[i][1]];
                let mut emails = Vec::new();
                while !stack.is_empty() {
                    let email = stack.pop().unwrap();
                    if visited.insert(email) {
                        emails.push(email.to_string());
                        if let Some(nexts) = graph.get(email) {
                            for next in nexts.iter() {
                                stack.push(next);
                            }
                        }
                    }
                }
                emails.sort_unstable();
                let mut row = vec![mail2account.get(&accounts[i][1]).unwrap().to_string()];
                row.append(&mut emails);
                res.push(row);
            }
        }

        res
    }
}

struct UnionFindSet<'a> {
    parent: HashMap<&'a str, &'a str>,
    rank: HashMap<&'a str, usize>,
}

impl<'a> UnionFindSet<'a> {
    fn new() -> Self {
        Self {
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }
    fn get_parent(&mut self, t: &'a str) -> &'a str {
        // avoided err: Cannot borrow `*self` as mutable because it is also borrowed as immutable
        self.parent.get(t).unwrap_or(&t)
    }

    fn find(&mut self, t: &'a str) -> &'a str {
        let parent = self.get_parent(t);
        if self.parent.get(t).unwrap_or(&t) != &t {
            let group = self.find(parent);
            self.parent.insert(t, group);
        }
        self.get_parent(t)
    }

    fn union(&mut self, u: &'a str, v: &'a str) -> bool {
        let pu = self.find(u);
        let pv = self.find(v);
        if pu == pv {
            return false;
        }
        let ru = *self.rank.get(u).unwrap_or(&0);
        let rv = *self.rank.get(v).unwrap_or(&0);
        match ru.cmp(&rv) {
            std::cmp::Ordering::Less => self.parent.insert(pu, pv),
            std::cmp::Ordering::Greater => self.parent.insert(pv, pu),
            std::cmp::Ordering::Equal => {
                self.rank.entry(pv).or_insert(1);
                self.parent.insert(pu, pv)
            }
        };
        true
    }
}
// @lc code=end

pub struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn it_work() {
        assert_eq!(
            Solution::accounts_merge_dfs(vec![
                vec_string!("John", "johnsmith@mail.com", "john_newyork@mail.com"),
                vec_string!("John", "johnsmith@mail.com", "john00@mail.com"),
                vec_string!("Mary", "mary@mail.com"),
                vec_string!("John", "johnnybravo@mail.com"),
            ]),
            vec![
                vec_string!(
                    "John",
                    "john00@mail.com",
                    "john_newyork@mail.com",
                    "johnsmith@mail.com"
                ),
                vec_string!("Mary", "mary@mail.com"),
                vec_string!("John", "johnnybravo@mail.com"),
            ]
        );
    }
}
