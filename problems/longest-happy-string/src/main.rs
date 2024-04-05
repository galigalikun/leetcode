fn main() {
    assert_eq!(Solution::longest_diverse_string(1, 1, 7), "ccaccbcc");
    assert_eq!(Solution::longest_diverse_string(7, 1, 0), "aabaa");
}

struct Solution;
impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut a = a;
        let mut b = b;
        let mut c = c;
        let mut res = "".to_string();
        let mut last = ' ';
        while a > 0 || b > 0 || c > 0 {
            let mut next = ' ';
            if (a >= b && a >= c && last != 'a') || (b == 0 && c == 0) {
                next = 'a';
            } else if (b >= a && b >= c && last != 'b') || (a == 0 && c == 0) {
                next = 'b';
            } else if (c >= a && c >= b && last != 'c') || (a == 0 && b == 0) {
                next = 'c';
            }
            if next == 'a' {
                if a >= 2 {
                    res.push('a');
                    res.push('a');
                    a -= 2;
                } else {
                    res.push('a');
                    a -= 1;
                }
            } else if next == 'b' {
                if b >= 2 {
                    res.push('b');
                    res.push('b');
                    b -= 2;
                } else {
                    res.push('b');
                    b -= 1;
                }
            } else if next == 'c' {
                if c >= 2 {
                    res.push('c');
                    res.push('c');
                    c -= 2;
                } else {
                    res.push('c');
                    c -= 1;
                }
            }
            last = next;
        }
        return res;
    }
}
