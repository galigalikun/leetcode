fn main() {
    assert_eq!(Solution::min_swaps_couples(vec![0, 2, 1, 3]), 1);
    assert_eq!(Solution::min_swaps_couples(vec![3, 2, 0, 1]), 0);
}

struct Solution {}

impl Solution {
    pub fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
        let n = row.len();
        let mut pos = vec![0_usize; n];

        for (i, &person) in row.iter().enumerate() {
            pos[person as usize] = i;
        }

        let mut swaps = 0_i32;
        for i in (0..n).step_by(2) {
            let left = row[i] as usize;
            let partner = left ^ 1;

            if row[i + 1] as usize == partner {
                continue;
            }

            swaps += 1;

            let partner_pos = pos[partner];
            let right = row[i + 1] as usize;

            row.swap(i + 1, partner_pos);

            pos[right] = partner_pos;
            pos[partner] = i + 1;
        }

        swaps
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn sample_case_1() {
        assert_eq!(Solution::min_swaps_couples(vec![0, 2, 1, 3]), 1);
    }

    #[test]
    fn sample_case_2() {
        assert_eq!(Solution::min_swaps_couples(vec![3, 2, 0, 1]), 0);
    }

    #[test]
    fn already_paired() {
        assert_eq!(Solution::min_swaps_couples(vec![0, 1, 2, 3, 4, 5]), 0);
    }

    #[test]
    fn needs_multiple_swaps() {
        assert_eq!(Solution::min_swaps_couples(vec![0, 2, 4, 6, 7, 1, 3, 5]), 3);
    }
}
