fn main() {
    assert_eq!(
        Solution::min_mirror_pair_distance(vec![12, 21, 45, 33, 54]),
        1
    );
    assert_eq!(Solution::min_mirror_pair_distance(vec![120, 21]), 1);
    assert_eq!(Solution::min_mirror_pair_distance(vec![21, 120]), -1);
}

struct Solution;
impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut min_distance = i32::MAX;
        for (i, &num) in nums.iter().enumerate() {
            let mirror = num
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            if let Some(j) = nums.iter().position(|&x| x == mirror) {
                if i != j {
                    min_distance = min_distance.min((i as i32 - j as i32).abs());
                }
            }
        }
        if min_distance == i32::MAX {
            -1
        } else {
            min_distance
        }
    }
}
