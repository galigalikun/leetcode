fn main() {
    assert_eq!(Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
    assert_eq!(Solution::compute_area(-2, -2, 2, 2, 3, -4, 4, -3), 17);
    assert_eq!(Solution::compute_area(-2, -2, 2, 2, 1, 1, 3, 3), 19);
    assert_eq!(Solution::compute_area(-2, -2, 2, 2, -3, 1, -1, 3), 19);
    assert_eq!(Solution::compute_area(-2, -2, 2, 2, -3, -3, -1, -1), 19);
    assert_eq!(Solution::compute_area(-2, -2, 2, 2, 1, -3, 3, 3), 24);
    assert_eq!(Solution::compute_area(-2, -2, 2, 2, -3, 1, 3, 3), 24);
    assert_eq!(Solution::compute_area(-2, -2, 2, 2, -3, -3, -1, 3), 24);
    assert_eq!(Solution::compute_area(-3, -3, 3, -1, -2, -2, 2, 2), 24);
    assert_eq!(Solution::compute_area(1, -3, 3, 3, -2, -2, 2, 2), 24);
    assert_eq!(Solution::compute_area(-3, 1, 3, 3, -2, -2, 2, 2), 24);
    assert_eq!(Solution::compute_area(-2, -2, 2, 2, -1, 4, 1, 6), 20);
    assert_eq!(Solution::compute_area(-5, -5, -4, 3, -3, -3, 3, 3), 44);
    assert_eq!(Solution::compute_area(-5, 4, 0, 5, -3, -3, 3, 3), 41);
    assert_eq!(Solution::compute_area(-5, -2, 5, 1, -3, -3, 3, 3), 48);
    assert_eq!(Solution::compute_area(-3, -2, 5, 1, -3, -3, 3, 3), 42);
    assert_eq!(Solution::compute_area(-2, -5, 1, 0, -3, -3, 3, 3), 42);
    assert_eq!(Solution::compute_area(-2, 4, 1, 5, -3, -3, 3, 3), 39);
    assert_eq!(Solution::compute_area(4, -5, 5, 0, -3, -3, 3, 3), 41);
    assert_eq!(Solution::compute_area(4, -5, 5, 3, -3, -3, 3, 3), 44);
    assert_eq!(Solution::compute_area(4, -3, 5, 0, -3, -3, 3, 3), 39);
    assert_eq!(
        Solution::compute_area(-2286, -1386, -2284, -1380, -2284, -1384, -2273, -1375),
        111
    );
    assert_eq!(
        Solution::compute_area(-2279, -1386, -2271, -1380, -2284, -1384, -2273, -1375),
        123
    );
    assert_eq!(
        Solution::compute_area(-2284, -1382, -2271, -1378, -2284, -1384, -2273, -1375),
        107
    );
    assert_eq!(
        Solution::compute_area(0, 0, 50000, 40000, 0, 0, 50000, 40000),
        2000000000
    );
}

struct Solution {}
// use std::collections::HashMap;
impl Solution {
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        // let mut area: HashMap<(i32, i32), bool> = HashMap::new();
        // for x in a..c {
        //     for y in b..d {
        //         area.insert((x, y), true);
        //     }
        // }
        // for x in e..g {
        //     for y in f..h {
        //         area.insert((x, y), true);
        //     }
        // }
        // println!("debug area {}", area.len());
        if e >= a && f >= b && c >= g && d >= h {
            return (c - a).abs() * (d - b).abs();
        } else if a >= e && b >= f && g >= c && h >= d {
            return (g - e).abs() * (h - f).abs();
        } else if g >= c && d >= h && e >= a && b >= f && e <= c && h >= b {
            return (c - a).abs() * (d - b).abs() + (g - e).abs() * (h - f).abs()
                - (c - e).abs() * (h - b).abs();
        } else if g >= c && d >= h && e >= a && b < f && c >= e {
            return (c - a).abs() * (d - b).abs() + (g - e).abs() * (h - f).abs()
                - (c - e).abs() * (h - f).abs();
        } else if g >= c && d >= h && a > e && b >= f && h >= b {
            return (c - a).abs() * (d - b).abs() + (g - e).abs() * (h - f).abs()
                - (c - a).abs() * (h - b).abs();
        } else if g >= c && d >= h && a > e && b < f {
            return (c - a).abs() * (d - b).abs() + (g - e).abs() * (h - f).abs()
                - (c - a).abs() * (h - f).abs();
        } else if g >= c && d >= f && c >= e && e >= a && b >= f && h >= b {
            return (c - a).abs() * (d - b).abs() + (g - e).abs() * (h - f).abs()
                - (c - e).abs() * (d - b).abs();
        } else if g >= c && d >= f && c >= e && h >= b && a >= e {
            return (c - a).abs() * (d - b).abs() + (g - e).abs() * (h - f).abs()
                - (c - a).abs() * (d - f).abs();
        } else if g >= c && d >= f && c >= e && h >= b {
            return (c - a).abs() * (d - b).abs() + (g - e).abs() * (h - f).abs()
                - (c - e).abs() * (d - f).abs();
        } else if c >= g && h >= d && a >= e && d >= f && b >= f && g >= a {
            return (c - a).abs() * (d - b).abs() + (g - e).abs() * (h - f).abs()
                - (g - a).abs() * (d - b).abs();
        } else if c >= g && h >= d && a >= e && d >= f && h >= b && g >= a {
            return (c - a).abs() * (d - b).abs() + (g - e).abs() * (h - f).abs()
                - (g - a).abs() * (d - f).abs();
        } else if g >= a && a >= e && c >= g && h >= b && b >= f && d >= h {
            return (c - a).abs() * (d - b).abs() + (g - e).abs() * (h - f).abs()
                - (g - a).abs() * (h - b).abs();
        } else if c >= g && e >= a && h >= b && f >= b && h >= d && d >= f {
            return (c - a).abs() * (d - b).abs() + (g - e).abs() * (h - f).abs()
                - (g - e).abs() * (d - f).abs();
        } else if c >= g && a >= e && h >= b && f >= b && d >= h && g >= a {
            return (c - a).abs() * (d - b).abs() + (g - e).abs() * (h - f).abs()
                - (g - a).abs() * (h - f).abs();
        } else if c >= g && e >= a && h >= b && b >= f && d >= h {
            return (c - a).abs() * (d - b).abs() + (g - e).abs() * (h - f).abs()
                - (g - e).abs() * (h - b).abs();
        } else if c >= g && e >= a && h >= b && b >= f && h >= d {
            return (c - a).abs() * (d - b).abs() + (g - e).abs() * (h - f).abs()
                - (g - e).abs() * (d - b).abs();
        }
        return (c - a).abs() * (d - b).abs() + (g - e).abs() * (h - f).abs();
    }
}
