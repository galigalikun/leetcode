fn main() {
    assert_eq!(Solution::magical_string(6), 3);
    assert_eq!(Solution::magical_string(1), 1);
}

pub struct Solution {}
// https://www.tutorialspoint.com/magical-string-in-cplusplus
impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        // 1221121221221121122
        // 1 22 11 2 1 22 1 22 11 2 11 22
        if n > 3 {
            let mut arr = vec![0; n as usize];
            arr[0] = 1;
            arr[1] = 2;
            arr[2] = 2;
            let mut head = 2;
            let mut tail = 3;
            let mut num = 1;
            let mut result = 1;
            while tail < n {
                for _i in 0..arr[head] {
                    arr[tail as usize] = num;
                    if num == 1 && tail < n {
                        result += 1;
                    }
                    tail += 1;
                    if tail >= n {
                        break;
                    }
                }
                num ^= 3;
                head += 1;
            }

            return result;
        } else if n > 0 {
            return 1;
        } else {
            return 0;
        }
    }
}
