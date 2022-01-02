fn main() {
    // assert_eq!(Solution::nearest_palindromic("123".to_string()), "121");
    // assert_eq!(Solution::nearest_palindromic("1".to_string()), "0");
    // assert_eq!(Solution::nearest_palindromic("1213".to_string()), "1221");
    assert_eq!(Solution::nearest_palindromic("2147483647".to_string()), "2147447412");
}

// https://aaronice.gitbook.io/lintcode/string/find-the-closest-palindrome
struct Solution {}
impl Solution {
    fn helper(n: String, dir: bool) -> String {
        let k = n.len() / 2;
        let mut base = *(&n[0..n.len() - k].parse::<i32>().unwrap());
        base += if dir { 1 } else { -1 };
        if base == 0 {
            if k == 0 {
                return "0".to_string();
            }
            return "9".to_string();
        }
        let mut left = base.to_string();
        let mut right = left.chars().rev().collect::<String>();
        if k > left.len() {
            right += "9";
        }
        left += &right[right.len() - k..];
        return left;
    }
    pub fn nearest_palindromic(n: String) -> String {
        let mut pp = n.chars().collect::<Vec<_>>();
        let (mut i, mut j) = (0, pp.len() - 1);
        loop {
            pp[j] = pp[i];
            if i >= j {
                break;
            }
            i += 1;
            j -= 1;
        }
        let p = pp.iter().collect::<String>();
        let p_prev = Solution::helper(p.clone(), false);
        let p_next = Solution::helper(p.clone(), true);
        let num = n.parse::<i64>().unwrap();
        let current = p.parse::<i64>().unwrap();
        let prev = p_prev.parse::<i64>().unwrap();
        let next = p_next.parse::<i64>().unwrap();
        let (d1, d2, d3) = (
            (num - current).abs(),
            (num - prev).abs(),
            (num - next).abs(),
        );
        if num == current {
            if d2 <= d3 {
                return p_prev;
            } else {
                return p_next;
            }
        } else if num > current {
            if d1 <= d3 {
                return p;
            } else {
                return p_next;
            }
        } else {
            if d2 <= d1 {
                return p_prev;
            } else {
                return p;
            }
        }
    }
}
