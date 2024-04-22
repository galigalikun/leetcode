fn main() {
    assert_eq!(Solution::min_number_of_frogs("croakcroak".to_string()), 1);
    assert_eq!(Solution::min_number_of_frogs("crcoakroak".to_string()), 2);
    assert_eq!(Solution::min_number_of_frogs("croakcrook".to_string()), -1);
}

struct Solution;
impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut croak = vec![0; 5];
        let mut frogs = 0;
        let mut max_frogs = 0;
        for c in croak_of_frogs.chars() {
            let i = match c {
                'c' => 0,
                'r' => 1,
                'o' => 2,
                'a' => 3,
                'k' => 4,
                _ => return -1,
            };
            croak[i] += 1;
            if i == 0 {
                frogs += 1;
                max_frogs = max_frogs.max(frogs);
            } else if croak[i - 1] < croak[i] {
                return -1;
            } else if i == 4 {
                frogs -= 1;
            }
        }
        return if croak.iter().all(|&x| x == croak[0]) {
            max_frogs
        } else {
            -1
        };
    }
}
