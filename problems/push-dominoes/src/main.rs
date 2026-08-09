fn main() {
    assert_eq!(Solution::push_dominoes("RR.L".to_string()), "RR.L");
    assert_eq!(Solution::push_dominoes(".L.R...LR..L..".to_string()), "LL.RR.LLRRLL..");
}

struct Solution{}
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let bytes = dominoes.as_bytes();
        let n = bytes.len();
        let mut force = vec![0_i32; n];

        let mut right_force = 0_i32;
        for i in 0..n {
            match bytes[i] {
                b'R' => right_force = n as i32,
                b'L' => right_force = 0,
                _ => right_force = (right_force - 1).max(0),
            }
            force[i] += right_force;
        }

        let mut left_force = 0_i32;
        for i in (0..n).rev() {
            match bytes[i] {
                b'L' => left_force = n as i32,
                b'R' => left_force = 0,
                _ => left_force = (left_force - 1).max(0),
            }
            force[i] -= left_force;
        }

        force
            .into_iter()
            .map(|f| {
                if f > 0 {
                    'R'
                } else if f < 0 {
                    'L'
                } else {
                    '.'
                }
            })
            .collect()
    }
}
