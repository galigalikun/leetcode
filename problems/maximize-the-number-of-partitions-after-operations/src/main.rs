fn main() {
    assert_eq!(
        Solution::max_partitions_after_operations("accca".to_string(), 2),
        3
    );
    assert_eq!(
        Solution::max_partitions_after_operations("aabaab".to_string(), 3),
        1
    );
    assert_eq!(
        Solution::max_partitions_after_operations("xxyz".to_string(), 1),
        4
    );
}

struct Solution;
impl Solution {
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut i = 0;
        while i < n {
            let mut j = i;
            while j + 1 < n && s[j + 1] == s[i] {
                j += 1;
            }
            let len = j - i + 1;
            if len % (k as usize) != 0 {
                return -1;
            }
            i = j + 1;
        }
        n as i32
    }
}
