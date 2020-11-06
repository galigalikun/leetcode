fn main() {
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![2, 2, 3], vec![7]]
    );
}

pub struct Solution {}
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = candidates.len();
        let mut ret = Vec::new();
        for i in 0..len {
            let num = candidates[i];
            if target > num {
                if target / num > 1 {
                    for j in 1..=target / num {
                        let mut aaa = Vec::new();
                        aaa.push(num);
                        for k in i + 1..len {
                            if (target - num * j - candidates[k]) == 0 {
                                let mut bbb = aaa;
                                bbb.push(candidates[k]);
                                ret.push(bbb);
                                break;
                            }
                        }
                    }
                }
            } else if target == num {
                ret.push(vec![num]);
            }
        }
        return ret;
    }
}
