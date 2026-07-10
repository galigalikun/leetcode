use std::collections::HashMap;

fn main() {
    assert_eq!(Solution::num_rabbits(vec![1, 1, 2]), 5);
    assert_eq!(Solution::num_rabbits(vec![10, 10, 10]), 11);
}

struct Solution {}
impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for answer in answers {
            *counts.entry(answer).or_insert(0) += 1;
        }

        let mut total = 0;
        for (answer, count) in counts {
            let group_size = answer + 1;
            let groups = (count + group_size - 1) / group_size;
            total += groups * group_size;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_case_one() {
        assert_eq!(Solution::num_rabbits(vec![1, 1, 2]), 5);
    }

    #[test]
    fn example_case_two() {
        assert_eq!(Solution::num_rabbits(vec![10, 10, 10]), 11);
    }

    #[test]
    fn empty_answers_returns_zero() {
        assert_eq!(Solution::num_rabbits(vec![]), 0);
    }

    #[test]
    fn zero_answers_count_individual_rabbits() {
        assert_eq!(Solution::num_rabbits(vec![0, 0, 0]), 3);
    }

    #[test]
    fn partial_group_rounds_up() {
        assert_eq!(Solution::num_rabbits(vec![2, 2, 2, 2]), 6);
    }
}
