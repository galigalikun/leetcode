fn main() {
    assert_eq!(Solution::min_cost_to_move_chips(vec![1, 2, 3]), 1);
    assert_eq!(Solution::min_cost_to_move_chips(vec![2, 2, 2, 3, 3]), 2);
    assert_eq!(Solution::min_cost_to_move_chips(vec![1, 1000000000]), 1);
}

struct Solution;
impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut position = position;
        for i in 0..position.len() {
            if position[i] % 2 == 0 {
                position[i] = 0;
            } else {
                position[i] = 1;
            }
        }
        let mut even = 0;
        let mut odd = 0;
        for i in 0..position.len() {
            if position[i] == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }
        if even > odd {
            return odd;
        } else {
            return even;
        }
    }
}
