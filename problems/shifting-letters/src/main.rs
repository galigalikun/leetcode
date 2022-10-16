fn main() {
    assert_eq!(
        Solution::shifting_letters("abc".to_string(), vec![3, 5, 9]),
        "rpl"
    );
    assert_eq!(
        Solution::shifting_letters("aaa".to_string(), vec![1, 2, 3]),
        "gfd"
    );
    assert_eq!(
        Solution::shifting_letters("z".to_string(), vec![52]),
        "z"
    );
}

struct Solution;
impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut ans = vec![];
        for c in s.chars() {
            ans.push(c as i32);
        }
        for (i, &shift) in shifts.iter().enumerate() {
            for j in 0..=i {
                ans[j] = ans[j] + shift;
                loop {
                    if ans[j] >= 123 {
                        ans[j] -= 26;
                    } else {
                        break;
                    }
                }
            }
        }
        return ans
            .iter()
            .map(|&x| std::char::from_u32(x as u32).unwrap())
            .collect::<String>();
    }
}
