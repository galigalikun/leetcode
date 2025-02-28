fn main() {
    assert_eq!(
        Solution::can_choose(
            vec![vec![1, -1, -1], vec![3, -2, 0]],
            vec![1, -1, 0, 1, -1, -1, 3, -2, 0]
        ),
        true
    );
    assert_eq!(
        Solution::can_choose(
            vec![vec![10, -2], vec![1, 2, 3, 4]],
            vec![1, 2, 3, 4, 10, -2]
        ),
        false
    );
    assert_eq!(
        Solution::can_choose(
            vec![vec![1, 2, 3], vec![3, 4]],
            vec![7, 7, 1, 2, 3, 4, 7, 7]
        ),
        false
    );
    assert_eq!(
        Solution::can_choose(
            vec![vec![9099312, -7882487, -1441304, 6624042, -6043305]],
            vec![-1441304, 9099312, -7882487, -1441304, 6624042, -6043305, -1441304]
        ),
        true
    );
}

struct Solution;
impl Solution {
    pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
        let mut i = 0;
        for group in groups.iter() {
            let mut j = 0;
            while j < group.len() {
                if i >= nums.len() {
                    return false;
                }
                if group[j] == nums[i] {
                    j += 1;
                }
                i += 1;
            }
        }
        if i >= nums.len() {
            return true;
        }
        return false;
    }
}
