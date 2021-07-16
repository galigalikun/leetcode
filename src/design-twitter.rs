use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::time::SystemTime;
struct Twitter {
    tweet: HashMap<i32, Vec<(SystemTime, i32)>>,
    follow: HashMap<i32, HashSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Twitter {
            tweet: HashMap::new(),
            follow: HashMap::new(),
        }
    }

    /** Compose a new tweet. */
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        if let Some(t) = self.tweet.get_mut(&user_id) {
            (*t).push((SystemTime::now(), tweet_id));
        } else {
            self.tweet
                .insert(user_id, vec![(SystemTime::now(), tweet_id)]);
        }
    }

    fn get_feed(&self, user_id: i32) -> Vec<(SystemTime, i32)> {
        if let Some(t) = self.tweet.get(&user_id) {
            return t.to_vec();
        }
        return vec![];
    }

    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut feed = vec![];
        feed.append(&mut self.get_feed(user_id));
        if let Some(followee) = self.follow.get(&user_id) {
            for f in followee {
                feed.append(&mut self.get_feed(*f));
            }
        }

        feed.sort_by(|(a, _), (b, _)| match a.duration_since(*b) {
            Ok(_) => Ordering::Less,
            Err(_) => Ordering::Greater,
        });

        if feed.len() > 10 {
            return ((&feed.iter().map(|(_, m)| *m).collect::<Vec<_>>())[0..10]).to_vec();
        }
        return feed.iter().map(|(_, m)| *m).collect::<Vec<_>>();
    }

    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(f) = self.follow.get_mut(&follower_id) {
            (*f).insert(followee_id);
        } else {
            let mut h = HashSet::new();
            h.insert(followee_id);
            self.follow.insert(follower_id, h);
        }
    }

    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(f) = self.follow.get_mut(&follower_id) {
            (*f).retain(|&x| x != followee_id);
        }
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
fn main() {
    let mut obj = Twitter::new();
    obj.post_tweet(1, 5);
    let ret_2: Vec<i32> = obj.get_news_feed(1);
    assert_eq!(ret_2, vec![5]);
    obj.follow(1, 2);
    obj.post_tweet(2, 6);
    let ret_3: Vec<i32> = obj.get_news_feed(1);
    assert_eq!(ret_3, vec![6, 5]);
    obj.unfollow(1, 2);
    let ret_4: Vec<i32> = obj.get_news_feed(1);
    assert_eq!(ret_4, vec![5]);
}
