fn main() {
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![2, 2, 3], vec![7]]
    );
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 5], 8),
        vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
    );
}

pub struct Solution {}
impl Solution {
    fn re_combination_sum(
        candidates: &mut Vec<i32>,
        target: i32,
        index: Option<usize>,
    ) -> Vec<Vec<i32>> {
        let len = candidates.len();
        let mut result = Vec::new();
        for i in 0..len {
            if Some(i) == index {
                continue;
            }
            let num = candidates[i];
            if target >= num {
                if target % num == 0 {
                    let mut v = Vec::new();
                    for _ in 0..target / num {
                        v.push(num);
                    }
                    result.push(v);
                } else {
                    for j in 1..=target / num {
                        for r in Solution::re_combination_sum(candidates, target - num * j, Some(i))
                        {
                            let mut v = Vec::new();
                            for _ in 0..j {
                                v.push(num);
                            }
                            v.extend(r);
                            result.push(v);
                        }
                    }
                }
            }
        }
        return result;
    }
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut zzz = candidates;
        return Solution::re_combination_sum(&mut zzz, target, None);
    }
}
