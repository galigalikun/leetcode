fn main() {
    assert_eq!(Solution::mirror_reflection(2, 1), 2);
    assert_eq!(Solution::mirror_reflection(3, 1), 1);
}

struct Solution;
impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let g = Self::gcd(p, q);
        let p_reduced = p / g;
        let q_reduced = q / g;

        if p_reduced % 2 == 1 && q_reduced % 2 == 1 {
            1
        } else if p_reduced % 2 == 1 {
            0
        } else {
            2
        }
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn sample_cases() {
        assert_eq!(Solution::mirror_reflection(2, 1), 2);
        assert_eq!(Solution::mirror_reflection(3, 1), 1);
    }

    #[test]
    fn additional_cases() {
        assert_eq!(Solution::mirror_reflection(4, 2), 2);
        assert_eq!(Solution::mirror_reflection(5, 3), 1);
        assert_eq!(Solution::mirror_reflection(5, 4), 0);
    }
}
