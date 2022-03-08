fn main() {
    assert_eq!(Solution::predict_party_victory("RD".to_string()), "Radiant");
    assert_eq!(Solution::predict_party_victory("RDD".to_string()), "Dire");
    assert_eq!(Solution::predict_party_victory("DR".to_string()), "Dire");
    assert_eq!(Solution::predict_party_victory("DDRRR".to_string()), "Dire");
    assert_eq!(Solution::predict_party_victory("RDRDRDD".to_string()), "Radiant");
}

// https://dreamume.medium.com/leetcode-649-dota2-senate-9f8adff6526
struct Solution {}
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut data = vec![];
        let mut i = 0;
        loop {
            let mut j = i + 1;
            while j < senate.len() && senate.chars().nth(i) == senate.chars().nth(j) {
                j += 1;
            }
            data.push(if senate.chars().nth(i) == Some('R') {
                j as i64 - i as i64
            } else {
                i as i64 - j as i64
            });
            i = j;
            if i >= senate.len() {
                break;
            }
        }
        let mut remains_ban = 0;
        let mut ban = 0;
        while data.len() > 1 {
            let mut second = vec![];
            for i in 0..data.len() {
                if remains_ban != 0 {
                    if (remains_ban > 0) ^ (data[i] > 0) {
                        if (remains_ban + data[i]) == 0 {
                            remains_ban = 0;
                            continue;
                        } else if (remains_ban + data[i] > 0) ^ (remains_ban > 0) {
                            data[i] += remains_ban;
                            remains_ban = 0;
                        } else {
                            remains_ban += data[i];
                            continue;
                        }
                    }
                }
                if ban == 0 {
                    second.push(data[i]);
                    ban += data[i];
                } else {
                    if (ban + data[i]) == 0 {
                        ban = 0;
                    } else if (ban + data[i] > 0) ^ (second.last() > Some(&0)) {
                        ban += data[i];
                        second.push(ban);
                    } else {
                        ban += data[i];
                        if (data[i] > 0) ^ (second.last() > Some(&0)) {
                            let a = second.last_mut().unwrap();
                            *a += data[i];
                        }
                    }
                }
            }
            remains_ban = ban;
            ban = 0;
            let d = data;
            data = second;
            second = d;
        }
        return if data.first() > Some(&0) {
            "Radiant"
        } else {
            "Dire"
        }
        .to_string();
    }
}
