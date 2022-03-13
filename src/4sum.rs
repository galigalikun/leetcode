fn main() {
    assert_eq!(
        Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
        vec![vec![1, 0, -1, 0], vec![1, -1, -2, 2], vec![0, 0, -2, 2]]
    );
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..nums.len() {
            if let Some(m) = map.get_mut(&nums[i]) {
                (*m).push(i);
            } else {
                map.insert(nums[i], vec![i]);
            }
        }
        let mut result = Vec::new();
        let mut unique = HashMap::new();
        for i in 0..nums.len() - 2 {
            for j in 1..nums.len() - 1 {
                for k in 2..nums.len() {
                    if i == j {
                        continue;
                    }
                    if j == k {
                        continue;
                    }
                    if k == i {
                        continue;
                    }
                    let key = target - nums[i] - nums[j] - nums[k];
                    if let Some(list) = map.get(&key) {
                        let mut ok = false;
                        for &l in list {
                            if i != l && j != l && k != l {
                                ok = true;
                                break;
                            }
                        }

                        if ok {
                            let mut data = vec![nums[i], nums[j], nums[k], key];
                            data.sort();
                            if None == unique.get(&data) {
                                unique.insert(data, true);
                                result.push(vec![nums[i], nums[j], nums[k], key]);
                            }
                        }
                    }
                }
            }
        }
        return result;
    }
}
