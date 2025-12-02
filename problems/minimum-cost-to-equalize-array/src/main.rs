fn main() {
    assert_eq!(Solution::min_cost_to_equalize_array(vec![4, 1], 5, 2), 15);
    assert_eq!(
        Solution::min_cost_to_equalize_array(vec![2, 3, 3, 3, 5], 2, 1),
        6
    );
    assert_eq!(Solution::min_cost_to_equalize_array(vec![3, 5, 3], 1, 3), 4);
}

struct Solution;
impl Solution {
    pub fn min_cost_to_equalize_array(nums: Vec<i32>, cost1: i32, cost2: i32) -> i32 {
        return nums
            .iter()
            .map(|&x| {
                let cost_to_cost1 = (x - cost1).abs();
                let cost_to_cost2 = (x - cost2).abs();
                cost_to_cost1.min(cost_to_cost2)
            })
            .sum();
    }
}
