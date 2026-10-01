fn main() {
    assert_eq!(Solution::prison_after_n_days(vec![0,1,0,1,1,0,0,1], 7), vec![0,0,1,1,0,0,0,0]);
    assert_eq!(Solution::prison_after_n_days(vec![1,0,0,1,0,0,1,0], 1000000000), vec![0,0,1,1,1,1,1,0]);
}

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        let mut state = cells;
        let mut days_left = n;
        let mut seen: HashMap<Vec<i32>, i32> = HashMap::new();

        while days_left > 0 {
            if let Some(&previous_days_left) = seen.get(&state) {
                let cycle_len = previous_days_left - days_left;
                if cycle_len > 0 {
                    days_left %= cycle_len;
                }
            }

            seen.insert(state.clone(), days_left);

            if days_left > 0 {
                days_left -= 1;
                state = Self::next_state(&state);
            }
        }

        state
    }

    fn next_state(cells: &[i32]) -> Vec<i32> {
        let mut next = vec![0; cells.len()];
        for i in 1..cells.len() - 1 {
            next[i] = i32::from(cells[i - 1] == cells[i + 1]);
        }
        next
    }
}
