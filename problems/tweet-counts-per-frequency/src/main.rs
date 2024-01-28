struct TweetCounts {
    data: std::collections::HashMap<String, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TweetCounts {
    fn new() -> Self {
        TweetCounts {
            data: std::collections::HashMap::new(),
        }
    }

    fn record_tweet(&mut self, tweet_name: String, time: i32) {
        let v = self.data.entry(tweet_name).or_insert(vec![]);
        v.push(time);
    }

    fn get_tweet_counts_per_frequency(
        &self,
        freq: String,
        tweet_name: String,
        start_time: i32,
        end_time: i32,
    ) -> Vec<i32> {
        let mut v = self.data.get(&tweet_name).unwrap().clone();
        v.sort();
        let mut ret = vec![];
        let mut start = start_time;
        let mut i = 0;
        while start <= end_time {
            let end = start
                + match freq.as_str() {
                    "minute" => 60,
                    "hour" => 3600,
                    "day" => 86400,
                    _ => 0,
                };
            let mut count = 0;
            while i < v.len() && v[i] < end {
                if v[i] >= start {
                    count += 1;
                }
                i += 1;
            }
            ret.push(count);
            start = end;
        }
        return ret;
    }
}

/**
 * Your TweetCounts object will be instantiated and called as such:
 * let obj = TweetCounts::new();
 * obj.record_tweet(tweetName, time);
 * let ret_2: Vec<i32> = obj.get_tweet_counts_per_frequency(freq, tweetName, startTime, endTime);
 */
fn main() {
    let mut obj = TweetCounts::new();
    obj.record_tweet("tweet3".to_string(), 0);
    obj.record_tweet("tweet3".to_string(), 60);
    obj.record_tweet("tweet3".to_string(), 10);
    let ret_2: Vec<i32> =
        obj.get_tweet_counts_per_frequency("minute".to_string(), "tweet3".to_string(), 0, 59);
    assert_eq!(vec![2], ret_2);
    let ret_2: Vec<i32> =
        obj.get_tweet_counts_per_frequency("minute".to_string(), "tweet3".to_string(), 0, 60);
    assert_eq!(vec![2, 1], ret_2);
    obj.record_tweet("tweet3".to_string(), 120);
    let ret_2: Vec<i32> =
        obj.get_tweet_counts_per_frequency("hour".to_string(), "tweet3".to_string(), 0, 210);
    assert_eq!(vec![4], ret_2);
}
