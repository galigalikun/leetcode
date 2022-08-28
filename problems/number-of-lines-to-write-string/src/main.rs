fn main() {
    assert_eq!(
        Solution::number_of_lines(
            vec![
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10
            ],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        ),
        vec![3, 60]
    );
    assert_eq!(
        Solution::number_of_lines(
            vec![
                4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10
            ],
            "bbbcccdddaaa".to_string()
        ),
        vec![2, 4]
    );

    assert_eq!(
        Solution::number_of_lines(
            vec![3, 4, 10, 4, 8, 7, 3, 3, 4, 9, 8, 2, 9, 6, 2, 8, 4, 9, 9, 10, 2, 4, 9, 10, 8, 2],
            "mqblbtpvicqhbrejb".to_string()
        ),
        vec![1, 100]
    );
}

struct Solution {}
impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut n = 0;
        let mut ans = vec![0, 0];
        for c in s.chars() {
            let idx = c as usize - 97;
            let width = widths[idx];
            n += width;
            if n > 100 {
                ans[0] += 1;
                n = width;
            } else if n == 100 {
                ans[0] += 1;
                ans[1] = n;
                n = 0;
            }
        }
        if n > 0 {
            ans[1] = n;
            ans[0] += 1;
        }

        return ans;
    }
}
