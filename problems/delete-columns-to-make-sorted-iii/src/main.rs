fn main() {
    assert_eq!(Solution::min_deletion_size(vec!["babca".to_string(),"bbazb".to_string()]), 3);
    assert_eq!(Solution::min_deletion_size(vec!["edcba".to_string()]), 4);
    assert_eq!(Solution::min_deletion_size(vec!["ghi".to_string(),"def".to_string(),"abc".to_string()]), 0);
}

struct Solution;
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let m = strs[0].len();
        let mut count = 0;
        for j in 0..m {
            for i in 1..n {
                if strs[i].as_bytes()[j] < strs[i - 1].as_bytes()[j] {
                    count += 1;
                    break;
                }
            }
        }
        count
    }
}
