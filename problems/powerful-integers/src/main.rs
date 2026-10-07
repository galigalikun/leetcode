fn main() {
    assert_eq!(Solution::powerful_integers(2, 3, 10), vec![2, 3, 4, 5, 7, 9, 10]);
    assert_eq!(Solution::powerful_integers(3, 5, 15), vec![2, 4, 6, 8, 10, 14]);
}

struct Solution;
impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        use std::collections::HashSet;

        if bound < 2 {
            return vec![];
        }

        let mut x_pows = Vec::new();
        let mut xp = 1;
        loop {
            if xp > bound {
                break;
            }
            x_pows.push(xp);
            if x == 1 {
                break;
            }
            xp *= x;
        }

        let mut y_pows = Vec::new();
        let mut yp = 1;
        loop {
            if yp > bound {
                break;
            }
            y_pows.push(yp);
            if y == 1 {
                break;
            }
            yp *= y;
        }

        let mut set = HashSet::new();
        for &a in &x_pows {
            for &b in &y_pows {
                let sum = a + b;
                if sum <= bound {
                    set.insert(sum);
                }
            }
        }

        let mut ans: Vec<i32> = set.into_iter().collect();
        ans.sort_unstable();
        ans
    }
}
