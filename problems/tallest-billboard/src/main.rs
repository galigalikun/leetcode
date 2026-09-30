fn main() {
    assert_eq!(Solution::tallest_billboard(vec![1, 2, 3, 6]), 6);
    assert_eq!(Solution::tallest_billboard(vec![1, 2, 3, 4, 5, 6]), 10);
    assert_eq!(Solution::tallest_billboard(vec![1, 2]), 0);
}

struct Solution;
impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let total: usize = rods.iter().map(|&rod| rod as usize).sum();
        // best[diff] is the greatest shorter height for this height difference.
        // A negative value marks an unreachable difference.
        let mut best = vec![-1; total + 1];
        best[0] = 0;
        let mut processed = 0;

        for rod in rods {
            let length = rod as usize;
            // Preserve skipping this rod, and read only states from before it.
            let mut next = best.clone();
            for diff in 0..=processed {
                let shorter = best[diff];
                if shorter < 0 {
                    continue;
                }

                // Add the rod to the taller support.
                next[diff + length] = next[diff + length].max(shorter);

                // Add it to the shorter support, possibly making it taller.
                let new_diff = diff.abs_diff(length);
                let new_shorter = shorter + diff.min(length) as i32;
                next[new_diff] = next[new_diff].max(new_shorter);
            }
            best = next;
            processed += length;
        }

        best[0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn handles_unused_rods_and_equal_rods() {
        assert_eq!(Solution::tallest_billboard(vec![1, 2, 3, 100]), 3);
        assert_eq!(Solution::tallest_billboard(vec![5, 5, 5]), 5);
    }

    #[test]
    fn handles_too_few_rods() {
        assert_eq!(Solution::tallest_billboard(vec![]), 0);
        assert_eq!(Solution::tallest_billboard(vec![7]), 0);
    }
}
