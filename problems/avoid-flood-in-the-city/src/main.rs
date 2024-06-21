fn main() {
    assert_eq!(
        Solution::avoid_flood(vec![1, 2, 3, 4]),
        vec![-1, -1, -1, -1]
    );
    assert_eq!(
        Solution::avoid_flood(vec![1, 2, 0, 0, 2, 1]),
        vec![-1, -1, 2, 1, -1, -1]
    );
    assert_eq!(Solution::avoid_flood(vec![1, 2, 0, 1, 2]), vec![]);
}

struct Solution;
impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![-1; rains.len()];
        let mut last_rain = std::collections::HashMap::new();
        let mut dry_days = std::collections::BTreeSet::new();
        for (i, &rain) in rains.iter().enumerate() {
            if rain == 0 {
                dry_days.insert(i);
                ans[i] = 1;
            } else {
                if let Some(&j) = last_rain.get(&rain) {
                    let dry_day = dry_days.range(j..).next().cloned();
                    if let Some(d) = dry_day {
                        ans[d] = rain;
                        dry_days.remove(&d);
                    } else {
                        return vec![];
                    }
                }
                last_rain.insert(rain, i);
            }
        }
        return ans;
    }
}
