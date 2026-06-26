fn main() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    assert_eq!(
        Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
        6
    );
}

struct Solution {}

impl Solution {
    /// Returns the minimum cost to reach the top of the floor (one step past
    /// the last index). You may start at index 0 or 1, and each paid step lets
    /// you climb one or two steps.
    ///
    /// `prev` and `curr` track the minimum cost to reach the previous two
    /// positions; the top is reached when the window slides past the array.
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (prev, curr) = cost.iter().fold((0, 0), |(prev, curr), &step| {
            (curr, std::cmp::min(prev, curr) + step)
        });
        std::cmp::min(prev, curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }

    #[test]
    fn two_steps() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![0, 0]), 0);
        assert_eq!(Solution::min_cost_climbing_stairs(vec![5, 3]), 3);
    }
}
