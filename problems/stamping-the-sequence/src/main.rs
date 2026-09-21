fn main() {
    let stamp = "abc".to_string();
    let target = "ababc".to_string();
    let moves = Solution::moves_to_stamp(stamp.clone(), target.clone());
    assert!(is_valid_moves(&stamp, &target, &moves));

    let stamp = "abca".to_string();
    let target = "aabcaca".to_string();
    let moves = Solution::moves_to_stamp(stamp.clone(), target.clone());
    assert!(is_valid_moves(&stamp, &target, &moves));
}

fn is_valid_moves(stamp: &str, target: &str, moves: &[i32]) -> bool {
    let stamp = stamp.as_bytes();
    let target = target.as_bytes();
    let m = stamp.len();
    let n = target.len();

    if moves.len() > 10 * n {
        return false;
    }

    let mut built = vec![b'?'; n];
    for &start in moves {
        if start < 0 {
            return false;
        }
        let start = start as usize;
        if start + m > n {
            return false;
        }
        built[start..(m + start)].copy_from_slice(&stamp[..m]);
    }

    built == target
}

struct Solution;
impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let stamp = stamp.as_bytes();
        let mut target = target.into_bytes();
        let m = stamp.len();
        let n = target.len();

        if m > n {
            return vec![];
        }

        let mut visited = vec![false; n - m + 1];
        let mut answer = Vec::new();
        let mut replaced_total = 0usize;

        while replaced_total < n {
            let mut changed = false;

            for i in 0..=n - m {
                if visited[i] || !Self::can_stamp(&target, stamp, i) {
                    continue;
                }

                visited[i] = true;
                let replaced = Self::apply_stamp(&mut target, i, m);
                if replaced > 0 {
                    changed = true;
                    replaced_total += replaced;
                    answer.push(i as i32);
                    if replaced_total == n {
                        break;
                    }
                }
            }

            if !changed {
                return vec![];
            }
        }

        answer.reverse();
        answer
    }

    fn can_stamp(target: &[u8], stamp: &[u8], start: usize) -> bool {
        for i in 0..stamp.len() {
            if target[start + i] != b'?' && target[start + i] != stamp[i] {
                return false;
            }
        }
        true
    }

    fn apply_stamp(target: &mut [u8], start: usize, len: usize) -> usize {
        let mut replaced = 0usize;
        for i in 0..len {
            if target[start + i] != b'?' {
                target[start + i] = b'?';
                replaced += 1;
            }
        }
        replaced
    }
}
