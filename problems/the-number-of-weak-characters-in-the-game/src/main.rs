fn main() {
    assert_eq!(
        Solution::number_of_weak_characters(vec![vec![5, 5], vec![6, 3], vec![3, 6]]),
        0
    );
    assert_eq!(
        Solution::number_of_weak_characters(
            vec![[2, 2], [3, 3]].iter().map(|v| v.to_vec()).collect()
        ),
        1
    );
    assert_eq!(
        Solution::number_of_weak_characters(
            vec![[1, 5], [10, 4], [4, 3]]
                .iter()
                .map(|v| v.to_vec())
                .collect()
        ),
        1
    );
}

struct Solution;
impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let mut properties = properties;
        properties.sort_unstable_by(|a, b| {
            if a[0] == b[0] {
                return a[1].cmp(&b[1]);
            }
            a[0].cmp(&b[0])
        });
        let mut weak_characters = 0;
        let mut max_defense = i32::MIN;
        for i in (0..properties.len()).rev() {
            if properties[i][1] < max_defense {
                weak_characters += 1;
            } else {
                max_defense = properties[i][1];
            }
        }
        weak_characters
    }
}
