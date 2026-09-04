fn main() {
    assert_eq!(Solution::orderly_queue("cba".to_string(), 1), "acb");
    assert_eq!(Solution::orderly_queue("baaca".to_string(), 3), "aaabc");
}

struct Solution;
impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        if k > 1 {
            let mut bytes = s.into_bytes();
            bytes.sort_unstable();
            return String::from_utf8(bytes).unwrap();
        }

        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut best_start = 0;

        for start in 1..n {
            for offset in 0..n {
                let current = bytes[(start + offset) % n];
                let best = bytes[(best_start + offset) % n];

                if current < best {
                    best_start = start;
                    break;
                }
                if current > best {
                    break;
                }
            }
        }

        let mut result = Vec::with_capacity(n);
        for offset in 0..n {
            result.push(bytes[(best_start + offset) % n]);
        }

        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn returns_smallest_rotation_when_k_is_one() {
        assert_eq!(Solution::orderly_queue("cba".to_string(), 1), "acb");
        assert_eq!(Solution::orderly_queue("daily".to_string(), 1), "ailyd");
    }

    #[test]
    fn returns_sorted_string_when_k_is_greater_than_one() {
        assert_eq!(Solution::orderly_queue("baaca".to_string(), 3), "aaabc");
        assert_eq!(Solution::orderly_queue("dcab".to_string(), 2), "abcd");
    }
}
