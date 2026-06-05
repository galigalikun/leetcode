fn main() {
    assert_eq!(Solution::judge_point24(vec![4, 1, 8, 7]), true);
    assert_eq!(Solution::judge_point24(vec![1, 2, 1, 2]), false);
}

struct Solution {}
impl Solution {
    const TARGET: f64 = 24.0;
    const EPSILON: f64 = 1e-6;

    fn is_24(value: f64) -> bool {
        (value - Self::TARGET).abs() < Self::EPSILON
    }

    fn judge_point24_helper(nums: Vec<f64>) -> bool {
        if nums.len() == 1 {
            return Self::is_24(nums[0]);
        }

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                let a = nums[i];
                let b = nums[j];

                let mut remaining = Vec::with_capacity(nums.len() - 1);
                for (idx, &num) in nums.iter().enumerate() {
                    if idx != i && idx != j {
                        remaining.push(num);
                    }
                }

                let mut candidates = vec![a + b, a - b, b - a, a * b];
                if b.abs() > Self::EPSILON {
                    candidates.push(a / b);
                }
                if a.abs() > Self::EPSILON {
                    candidates.push(b / a);
                }

                for candidate in candidates {
                    let mut next = remaining.clone();
                    next.push(candidate);
                    if Self::judge_point24_helper(next) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let nums: Vec<f64> = cards.into_iter().map(|card| card as f64).collect();
        Self::judge_point24_helper(nums)
    }
}
