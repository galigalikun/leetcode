fn main() {
    assert_eq!(
        Solution::min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7]),
        2
    );
    assert_eq!(Solution::min_set_size(vec![7, 7, 7, 7, 7, 7]), 1);
}

struct Solution;
impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let cnt = arr.len() / 2;
        let mut map = std::collections::HashMap::new();
        for i in arr {
            let count = map.entry(i).or_insert(0);
            *count += 1;
        }
        let mut v: Vec<_> = map.iter().collect();
        v.sort_by(|a, b| b.1.cmp(a.1));
        let mut sum = 0;
        let mut count = 0;
        for i in v {
            sum += i.1;
            count += 1;
            if sum >= cnt {
                return count;
            }
        }
        return 0;
    }
}
