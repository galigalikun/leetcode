fn main() {
    assert_eq!(
        Solution::closest_target(
            vec![
                "hello".to_string(),
                "i".to_string(),
                "am".to_string(),
                "leetcode".to_string()
            ],
            "hello".to_string(),
            1
        ),
        1
    );
    assert_eq!(
        Solution::closest_target(
            vec!["a".to_string(), "b".to_string(), "leetcode".to_string()],
            "leetcode".to_string(),
            0
        ),
        1
    );
    assert_eq!(
        Solution::closest_target(
            vec!["i".to_string(), "eat".to_string(), "leetcode".to_string()],
            "ate".to_string(),
            0
        ),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let n = words.len() as i32;
        let mut min_distance = i32::MAX;
        for i in 0..n {
            if words[i as usize] == target {
                let distance = (i - start_index).abs().min(n - (i - start_index).abs());
                min_distance = min_distance.min(distance);
            }
        }
        if min_distance == i32::MAX {
            -1
        } else {
            min_distance
        }
    }
}
