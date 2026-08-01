use std::convert::TryFrom;

fn main() {
    assert_eq!(Solution::racecar(3), 2);
    assert_eq!(Solution::racecar(6), 5);
}

struct Solution {}
impl Solution {
    pub fn racecar(target: i32) -> i32 {
        let target_usize = usize::try_from(target).expect("target must be non-negative");
        let mut memo = vec![-1; target_usize + 1];
        Self::solve(target_usize, &mut memo)
    }

    fn solve(target: usize, memo: &mut [i32]) -> i32 {
        if target == 0 {
            return 0;
        }

        if memo[target] != -1 {
            return memo[target];
        }

        let mut steps = 1usize;
        while (1usize << steps) - 1 < target {
            steps += 1;
        }

        if (1usize << steps) - 1 == target {
            let result = i32::try_from(steps).expect("steps fits in i32");
            memo[target] = result;
            return result;
        }

        let overshoot_distance = (1usize << steps) - 1 - target;
        let overshoot = i32::try_from(steps + 1).expect("steps fits in i32")
            + Self::solve(overshoot_distance, memo);

        let mut best = overshoot;

        let forward_steps = steps - 1;
        for backward_steps in 0..forward_steps {
            let forward_distance = (1usize << forward_steps) - 1;
            let backward_distance = (1usize << backward_steps) - 1;
            let remaining = target - forward_distance + backward_distance;

            let candidate = i32::try_from(forward_steps + 1 + backward_steps + 1)
                .expect("steps fits in i32")
                + Self::solve(remaining, memo);
            best = best.min(candidate);
        }

        memo[target] = best;
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_cases() {
        assert_eq!(Solution::racecar(3), 2);
        assert_eq!(Solution::racecar(6), 5);
    }

    #[test]
    fn additional_cases() {
        assert_eq!(Solution::racecar(1), 1);
        assert_eq!(Solution::racecar(2), 4);
        assert_eq!(Solution::racecar(4), 5);
        assert_eq!(Solution::racecar(5), 7);
        assert_eq!(Solution::racecar(9), 8);
    }
}
