fn main() {
    assert_eq!(
        Solution::split_into_fibonacci("1101111".to_string()),
        vec![11, 0, 11, 11]
    );
    assert_eq!(
        Solution::split_into_fibonacci("112358130".to_string()),
        vec![]
    );
    assert_eq!(Solution::split_into_fibonacci("0123".to_string()), vec![]);
}

struct Solution {}
impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        let bytes = num.as_bytes();
        let mut sequence = Vec::new();

        fn dfs(index: usize, bytes: &[u8], sequence: &mut Vec<i32>) -> bool {
            if index == bytes.len() {
                return sequence.len() >= 3;
            }

            let mut value: i64 = 0;
            for i in index..bytes.len() {
                if i > index && bytes[index] == b'0' {
                    break;
                }

                value = value * 10 + (bytes[i] - b'0') as i64;
                if value > i32::MAX as i64 {
                    break;
                }

                let current = value as i32;
                let len = sequence.len();
                if len >= 2 {
                    let sum = sequence[len - 1] as i64 + sequence[len - 2] as i64;
                    if value < sum {
                        continue;
                    }
                    if value > sum {
                        break;
                    }
                }

                sequence.push(current);
                if dfs(i + 1, bytes, sequence) {
                    return true;
                }
                sequence.pop();
            }

            false
        }

        if dfs(0, bytes, &mut sequence) {
            sequence
        } else {
            vec![]
        }
    }
}
