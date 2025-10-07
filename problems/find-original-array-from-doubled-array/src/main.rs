fn main() {
    assert_eq!(Solution::find_original_array(vec![1, 3, 4, 2, 6, 8]), vec![1, 3, 4]);
    assert_eq!(Solution::find_original_array(vec![6, 3, 0, 1, 2, 4]), vec![0, 1, 2, 3]);
    assert_eq!(Solution::find_original_array(vec![1]), vec![]);
}

struct Solution;
impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        if changed.len() % 2 != 0 {
            return vec![];
        }
        let mut count = std::collections::HashMap::new();
        for &num in &changed {
            *count.entry(num).or_insert(0) += 1;
        }
        let mut original = Vec::new();
        let mut keys: Vec<i32> = count.keys().cloned().collect();
        keys.sort_unstable();
        for &num in &keys {
            if let Some(&cnt) = count.get(&num) {
                if cnt == 0 {
                    continue;
                }
                let double = num * 2;
                if let Some(&double_cnt) = count.get(&double) {
                    if double_cnt < cnt {
                        return vec![];
                    }
                    for _ in 0..cnt {
                        original.push(num);
                    }
                    *count.get_mut(&double).unwrap() -= cnt;
                } else {
                    return vec![];
                }
            }
        }
        original
    }
}
