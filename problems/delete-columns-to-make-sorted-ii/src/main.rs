fn main() {
    assert_eq!(Solution::min_deletion_size(vec!["ca".to_string(),"bb".to_string(),"ac".to_string()]), 1);
    assert_eq!(Solution::min_deletion_size(vec!["xc".to_string(),"yb".to_string(),"za".to_string()]), 0);
    assert_eq!(Solution::min_deletion_size(vec!["zyx".to_string(),"wvu".to_string(),"tsr".to_string()]), 3);
}

struct Solution;
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n = strs.len();
        if n <= 1 {
            return 0;
        }

        let m = strs[0].len();
        let rows: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();
        let mut sorted = vec![false; n - 1];
        let mut deletions = 0;

        for col in 0..m {
            let mut should_delete = false;
            for i in 0..n - 1 {
                if !sorted[i] && rows[i][col] > rows[i + 1][col] {
                    should_delete = true;
                    break;
                }
            }

            if should_delete {
                deletions += 1;
                continue;
            }

            for i in 0..n - 1 {
                if !sorted[i] && rows[i][col] < rows[i + 1][col] {
                    sorted[i] = true; // This pair is now strictly ordered.
                }
            }
        }

        deletions
    }
}
