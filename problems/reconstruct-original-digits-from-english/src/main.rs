fn main() {
    assert_eq!(Solution::original_digits("owoztneoer".to_string()), "012");
    assert_eq!(Solution::original_digits("fviefuro".to_string()), "45");
}

struct Solution {}
// https://dev.to/seanpgallivan/solution-reconstruct-original-digits-from-english-4o2p
impl Solution {
    pub fn original_digits(s: String) -> String {
        let digits = vec![
            (0, 25, vec![14]),
            (2, 22, vec![14]),
            (4, 20, vec![5, 14]),
            (6, 23, vec![18, 8]),
            (8, 6, vec![8, 7]),
            (5, 5, vec![8]),
            (7, 18, vec![]),
            (3, 7, vec![]),
            (9, 8, vec![]),
            (1, 14, vec![]),
        ];
        let mut fmap = vec![0; 26];
        for c in s.chars() {
            fmap[c as usize - 97] += 1;
        }
        let mut result = vec!["".to_string(); 10];
        for i in 0..10 {
            let (dig, char, rems) = &digits[i];
            let count = fmap[*char as usize];
            for &rem in rems {
                fmap[rem as usize] -= count;
            }
            result[*dig as usize] = std::iter::repeat(format!("{}", *dig))
                .take(count)
                .collect::<_>()
        }
        return result.into_iter().collect::<String>();
    }
}
