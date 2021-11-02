fn main() {
    assert_eq!(Solution::strong_password_checker("a".to_string()), 5);
    assert_eq!(Solution::strong_password_checker("aA1".to_string()), 3);
    assert_eq!(Solution::strong_password_checker("1337C0d3".to_string()), 0);
    assert_eq!(Solution::strong_password_checker("aaa123".to_string()), 1);
    assert_eq!(
        Solution::strong_password_checker("ABABABABABABABABABAB1".to_string()),
        2
    );
    assert_eq!(
        Solution::strong_password_checker("bbaaaaaaaaaaaaaaacccccc".to_string()),
        8
    );
}

pub struct Solution {}
impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let mut result = 0;
        let mut lowercase = 1;
        let mut uppercase = 1;
        let mut digest = 1;
        let mut arr = vec![0; password.len()];

        let mut i = 0;
        while i < password.len() {
            let ascii = password.chars().nth(i).unwrap() as u8;
            match ascii {
                48..=57 => digest = 0,
                65..=90 => uppercase = 0,
                97..=122 => lowercase = 0,
                _ => {}
            }
            let j = i;
            while i < password.len() && password.chars().nth(i) == password.chars().nth(j) {
                i += 1;
            }
            arr[j] = i as i32 - j as i32;
        }

        let total_missing = digest + lowercase + uppercase;

        if password.len() < 6 {
            result += total_missing + std::cmp::max(0, 6 - (arr.len() as i32 + total_missing));
        } else {
            let mut over_len = if arr.len() > 20 {
                arr.len() as i32 - 20
            } else {
                0
            };
            let mut left_over: i32 = 0;
            result += over_len;

            for k in 1..3 {
                let mut i = 0;
                while i < arr.len() && over_len > 0 {
                    if arr[i] < 3 || arr[i] % 3 != (k - 1) {
                        i += 1;
                        continue;
                    }
                    arr[i] -= std::cmp::min(over_len, k);
                    over_len -= k;
                    i += 1;
                }
            }

            for i in 0..arr.len() {
                if arr[i] >= 3 && over_len > 0 {
                    let need = arr[i] - 2;
                    arr[i] -= over_len;
                    over_len -= need;
                }
                if arr[i] >= 3 {
                    left_over += arr[i] / 3;
                }
            }

            result += std::cmp::max(total_missing, left_over);
        }
        return result;
    }
}
