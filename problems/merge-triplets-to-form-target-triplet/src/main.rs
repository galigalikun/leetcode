fn main() {
    assert_eq!(
        Solution::merge_triplets(
            vec![vec![2, 5, 3], vec![1, 8, 4], vec![1, 7, 5]],
            vec![2, 7, 5]
        ),
        true
    );
    assert_eq!(
        Solution::merge_triplets(vec![vec![3, 4, 5], vec![4, 5, 6]], vec![3, 2, 5]),
        false
    );
    assert_eq!(
        Solution::merge_triplets(
            vec![vec![2, 5, 3], vec![2, 3, 4], vec![1, 2, 5], vec![5, 2, 3]],
            vec![5, 5, 5]
        ),
        true
    );
}

struct Solution;
impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let mut valid = vec![false; 3];
        for triplet in &triplets {
            if triplet[0] > target[0] || triplet[1] > target[1] || triplet[2] > target[2] {
                continue;
            }
            for i in 0..3 {
                if triplet[i] == target[i] {
                    valid[i] = true;
                }
            }
        }
        valid.iter().all(|&v| v)
    }
}
