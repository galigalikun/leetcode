fn main() {
    // assert_eq!(Solution::find_integers(5), 5);
    // assert_eq!(Solution::find_integers(1), 2);
    // assert_eq!(Solution::find_integers(2), 3);
    assert_eq!(Solution::find_integers(6), 5);
}

struct Solution {}
impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        match n {
            0 | 1 | 2 => {
                return n+1;
            }
            3 => {
                return n;
            }
            _ => {
                let b = format!("{:b}", n);
                return n - if b.len() == b.chars().filter(|&c| c == '1').count() {
                    b.len() - 2
                } else {
                    b.len() - 3
                } as i32;
            }
        }
    }
}
