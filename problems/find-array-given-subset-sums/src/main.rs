fn main() {
    assert_eq!(
        Solution::recover_array(3, vec![-3, -2, -1, 0, 0, 1, 2, 3]),
        vec![1, 2, -3]
    );
    assert_eq!(Solution::recover_array(2, vec![0, 0, 0, 0]), vec![0, 0]);
    assert_eq!(
        Solution::recover_array(4, vec![0, 0, 5, 5, 4, -1, 4, 9, 9, -1, 4, 3, 4, 8, 3, 8]),
        vec![0, -1, 4, 5]
    );
}

struct Solution;
impl Solution {
    pub fn recover_array(n: i32, sums: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        let mut sums = sums;
        sums.sort_unstable();
        for i in 1..2 * n {
            let diff = sums[i] - sums[0];
            if diff <= 0 || diff % 2 != 0 {
                continue;
            }
            let x = diff / 2;
            let mut count = std::collections::HashMap::new();
            for &s in &sums {
                *count.entry(s).or_insert(0) += 1;
            }
            let mut result = vec![x];
            let mut next_sums = vec![];
            for &s in &sums {
                if let Some(c) = count.get_mut(&s) {
                    if *c > 0 {
                        *c -= 1;
                        if let Some(c2) = count.get_mut(&(s + 2 * x)) {
                            if *c2 > 0 {
                                *c2 -= 1;
                                next_sums.push(s + x);
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
            if next_sums.len() == n {
                result.extend(next_sums);
                return result;
            }
        }

        return vec![];
    }
}
