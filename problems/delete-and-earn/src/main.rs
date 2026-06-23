fn main() {
    assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
    assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
}

struct Solution {}
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        // Earning a value yields the sum of all its copies, but forbids
        // earning the adjacent values (v-1 and v+1). Bucketing the total
        // points per value turns this into the House Robber problem over
        // consecutive integers.
        let max = match nums.iter().max() {
            Some(&m) => m as usize,
            None => return 0,
        };

        let mut points = vec![0i32; max + 1];
        for n in nums {
            points[n as usize] += n;
        }

        // skip: best total ignoring value `v`; take: best total earning `v`.
        let (mut skip, mut take) = (0, 0);
        for &p in &points {
            let prev_skip = skip;
            skip = skip.max(take);
            take = prev_skip + p;
        }
        skip.max(take)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::delete_and_earn(vec![1]), 1);
    }

    #[test]
    fn empty() {
        assert_eq!(Solution::delete_and_earn(vec![]), 0);
    }

    #[test]
    fn all_same_value() {
        // Earning 3 takes every copy: 3 * 4 = 12.
        assert_eq!(Solution::delete_and_earn(vec![3, 3, 3, 3]), 12);
    }

    #[test]
    fn non_adjacent_values() {
        // 1 and 5 are not adjacent, so both can be earned: 1 + 5 = 6.
        assert_eq!(Solution::delete_and_earn(vec![1, 5]), 6);
    }
}
