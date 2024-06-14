fn main() {
    assert_eq!(Solution::find_least_num_of_unique_ints(vec![5, 5, 4], 1), 1);
    assert_eq!(
        Solution::find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3),
        2
    );
}

struct Solution;
impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut map = std::collections::HashMap::new();
        for &num in &arr {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut v: Vec<i32> = map.values().map(|&x| x).collect();
        v.sort();
        let mut k = k;
        for &num in &v {
            if k >= num {
                k -= num;
                continue;
            }
            return v.len() as i32 - k / num;
        }

        return 0;
    }
}
