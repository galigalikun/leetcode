fn main() {
    assert_eq!(Solution::last_remaining(9), 6);
    assert_eq!(Solution::last_remaining(1), 1);
    assert_eq!(Solution::last_remaining(4), 2);
    assert_eq!(Solution::last_remaining(10), 8);
    assert_eq!(Solution::last_remaining(1000000), 481110);
    assert_eq!(Solution::last_remaining(10000000), 6150102);
}

pub struct Solution {}
impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut result = (1..=n).collect::<Vec<_>>();
        let mut i = 0;
        while result.len() != 1 {
            if i % 2 == 0 {
                for j in 0..=result.len() / 2 {
                    if result.len() > j {
                        result.remove(j);
                    }
                    if result.len() == 1 {
                        break;
                    }
                }
            } else {
                let mut j = result.len() as i32 - 1;
                while j >= 0 {
                    result.remove(j as usize);
                    if result.len() == 1 {
                        break;
                    }
                    j -= 2;
                }
            }
            i += 1;
        }

        return result[0];
    }
}
