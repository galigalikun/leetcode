fn main() {
    assert_eq!(
        Solution::find_the_string(
            vec![[4, 0, 2, 0], [0, 3, 0, 1], [2, 0, 2, 0], [0, 1, 0, 1]]
                .iter()
                .map(|x| x.to_vec())
                .collect()
        ),
        "abab".to_string()
    );
    assert_eq!(
        Solution::find_the_string(
            vec![[4, 3, 2, 1], [3, 3, 2, 1], [2, 2, 2, 1], [1, 1, 1, 1]]
                .iter()
                .map(|x| x.to_vec())
                .collect()
        ),
        "aaaa".to_string()
    );
    assert_eq!(
        Solution::find_the_string(
            vec![[4, 3, 2, 1], [3, 3, 2, 1], [2, 2, 2, 1], [1, 1, 1, 3]]
                .iter()
                .map(|x| x.to_vec())
                .collect()
        ),
        "".to_string()
    );
}

struct Solution;
impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();
        let mut ans = vec![0; n];
        for i in 0..n {
            for j in 0..n {
                if lcp[i][j] > 0 {
                    ans[j] = ans[i];
                } else if ans[j] == ans[i] {
                    ans[j] = (ans.iter().max().unwrap() + 1) as i32;
                }
            }
        }
        if ans.iter().max().unwrap() >= &26 {
            return String::new();
        }
        ans.into_iter().map(|x| (b'a' + x as u8) as char).collect()
    }
}
