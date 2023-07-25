fn main() {
    assert_eq!(Solution::longest_wpi(vec![9, 9, 6, 0, 6, 6, 9]), 3);
    assert_eq!(Solution::longest_wpi(vec![6, 6, 6]), 0);
}

struct Solution;
impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut sum = 0;
        let mut map = std::collections::HashMap::new();
        for (i, h) in hours.iter().enumerate() {
            if *h > 8 {
                sum += 1;
            } else {
                sum -= 1;
            }
            if sum > 0 {
                max = i + 1;
            } else {
                if !map.contains_key(&sum) {
                    map.insert(sum, i);
                }
                if let Some(&j) = map.get(&(sum - 1)) {
                    max = max.max(i - j);
                }
            }
        }
        return max as i32;
    }
}
