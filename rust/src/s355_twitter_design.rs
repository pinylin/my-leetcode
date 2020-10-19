/*
 * @lc app=leetcode.cn id=355 lang=rust
 *
 * [355] 设计推特
refer: https://github.com/gwy15/leetcode/blob/d2a8969946848a56826299303b9999dbf93ff472/src/355.%E8%AE%BE%E8%AE%A1%E6%8E%A8%E7%89%B9.rs
 */

// @lc code=start
use std::collections::{BinaryHeap, HashMap, HashSet};

type User = i32;
// t, id
type Post = (u32, i32);

struct Twitter {
    t: u32,
    posts: HashMap<User, Vec<Post>>,
    follows: HashMap<User, HashSet<User>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Twitter {
            t: 0,
            posts: HashMap::new(),
            follows: HashMap::new(),
        }
    }
    /** Compose a new tweet. */
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        match self.posts.get_mut(&user_id) {
            None => {
                self.posts.insert(user_id, vec![(self.t, tweet_id)]);
            }
            Some(p) => p.push((self.t, tweet_id)),
        };
        self.t += 1;
    }
    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        // get follows
        let follows = match self.follows.get_mut(&user_id) {
            None => {
                self.follows.insert(user_id, HashSet::new());
                self.follows.get_mut(&user_id).unwrap()
            }
            Some(f) => f,
        };
        follows.insert(user_id);
        // insert follows into heap
        let mut heap = BinaryHeap::new();
        for followee_id in follows.iter() {
            match self.posts.get(followee_id) {
                None => None,
                Some(posts) => posts.last(),
            }
            .and_then(|&(t, post_id)| Some(heap.push((t, post_id, followee_id))));
        }
        // keep popping from heap
        let mut posts = Vec::new();
        for _ in 0..10 {
            match heap.pop() {
                None => break,
                Some((t, post_id, followee_id)) => {
                    posts.push(post_id);
                    // retrieve another from followee_id
                    self.posts
                        .get(followee_id)
                        .unwrap()
                        .iter()
                        .rev()
                        .filter(|&&(tt, _post_id)| tt < t)
                        .next()
                        .and_then(|&(t, post_id)| Some(heap.push((t, post_id, followee_id))));
                }
            }
        }
        posts
    }
    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        match self.follows.get_mut(&follower_id) {
            None => {
                let mut f = HashSet::new();
                f.insert(followee_id);
                self.follows.insert(follower_id, f);
            }
            Some(f) => {
                f.insert(followee_id);
            }
        };
    }
    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        match self.follows.get_mut(&follower_id) {
            None => {}
            Some(f) => {
                f.remove(&followee_id);
            }
        };
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Twitter;

    #[test]
    fn test_twitter() {
        let mut obj = Twitter::new();
        obj.post_tweet(1, 5);
        assert_eq!(vec![5], obj.get_news_feed(1));
        obj.follow(1, 2);
        obj.post_tweet(2, 6);
        assert_eq!(vec![6, 5], obj.get_news_feed(1));
        obj.unfollow(1, 2);
        assert_eq!(vec![5], obj.get_news_feed(1));
        assert_eq!(vec![6], obj.get_news_feed(2));
        assert_eq!(obj.get_news_feed(3), vec![]);
        obj.follow(2, 1);
        assert_eq!(vec![6, 5], obj.get_news_feed(2));
        obj.post_tweet(2, 7);
        assert_eq!(vec![7, 6, 5], obj.get_news_feed(2));
        assert_eq!(vec![5], obj.get_news_feed(1));

        {
            let mut obj = Twitter::new();
            let posts = vec![
                (1, 5),
                (1, 3),
                (1, 101),
                (1, 13),
                (1, 10),
                (1, 2),
                (1, 94),
                (1, 505),
                (1, 333),
            ];
            for (uid, tid) in posts {
                obj.post_tweet(uid, tid);
            }
            assert_eq!(
                vec![333, 505, 94, 2, 10, 13, 101, 3, 5],
                obj.get_news_feed(1)
            );
        }
        {
            let posts = vec![
                (1, 5),
                (1, 3),
                (1, 101),
                (1, 13),
                (1, 10),
                (1, 2),
                (1, 94),
                (1, 505),
                (1, 333),
                (1, 22),
                (1, 11),
            ];
            let mut obj = Twitter::new();
            for (uid, tid) in posts {
                obj.post_tweet(uid, tid);
            }
            assert_eq!(
                vec![11, 22, 333, 505, 94, 2, 10, 13, 101, 3],
                obj.get_news_feed(1)
            );
        }
    }
}
