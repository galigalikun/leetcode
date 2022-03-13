fn main() {
    assert_eq!(Solution::is_self_crossing(vec![2, 1, 1, 2]), true);

    assert_eq!(Solution::is_self_crossing(vec![1, 2, 3, 4]), false);

    assert_eq!(Solution::is_self_crossing(vec![1, 1, 1, 1]), true);
}

struct Solution {}
impl Solution {
    pub fn is_self_crossing(distance: Vec<i32>) -> bool {
        for i in 3..distance.len() {
            if distance[i] >= distance[i - 2] && distance[i - 3] >= distance[i - 1] {
                return true;
            }
            if i >= 4
                && distance[i - 1] == distance[i - 3]
                && distance[i] >= distance[i - 2] - distance[i - 4]
            {
                return true;
            }
            if i >= 5
                && distance[i - 2] >= distance[i - 4]
                && distance[i - 3] >= distance[i - 1]
                && distance[i - 1] >= distance[i - 3] - distance[i - 5]
                && distance[i] >= distance[i - 2] - distance[i - 4]
            {
                return true;
            }
        }
        return false;
    }
}
