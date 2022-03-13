fn main() {
    assert_eq!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]), true);
    assert_eq!(Solution::increasing_triplet(vec![5, 4, 3, 2, 1]), false);
    assert_eq!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]), true);
    assert_eq!(
        Solution::increasing_triplet(vec![20, 100, 10, 12, 5, 13]),
        true
    );
    assert_eq!(Solution::increasing_triplet(vec![1, 2, 1, 3]), true);
}

struct Solution {}
// https://medium.com/@xiaogegexiao/increasing-triplet-subsequence-problem-995471b338f1
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut min_one = std::i32::MAX;
        let mut min_two = std::i32::MAX;
        for n in nums {
            min_one = std::cmp::min(n, min_one);
            if n > min_one {
                min_two = std::cmp::min(n, min_two);
            }
            if n > min_two {
                return true;
            }
        }

        return false;
    }
}
