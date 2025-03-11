fn main() {
    assert_eq!(Solution::closest_cost(vec![1, 7], vec![3, 4], 10), 10);
    assert_eq!(Solution::closest_cost(vec![2,3], vec![4,5,100], 18), 17);
    assert_eq!(Solution::closest_cost(vec![3,10], vec![2,5], 9), 8);
}

struct Solution;
impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let mut closest = i128::MAX;
        let mut closest_diff = i128::MAX;
        for &base in &base_costs {
            let mut stack = vec![base as i128];
            while let Some(top) = stack.pop() {
                if (top - target as i128).abs() < closest_diff || (top - target as i128).abs() == closest_diff as i128 && top < closest as i128{
                    closest = top;
                    closest_diff = (top - target as i128).abs();
                }
                for &topping in &topping_costs {
                    stack.push(top + topping as i128);
                    stack.push(top + 2 * topping as i128);
                }
            }
        }
        closest as i32
    }
}
