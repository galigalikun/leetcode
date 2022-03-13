fn main() {
    assert_eq!(
        Solution::is_rectangle_cover(vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![3, 2, 4, 4],
            vec![1, 3, 2, 4],
            vec![2, 3, 3, 4]
        ]),
        true
    );
    assert_eq!(
        Solution::is_rectangle_cover(vec![
            vec![1, 1, 2, 3],
            vec![1, 3, 2, 4],
            vec![3, 1, 4, 2],
            vec![3, 2, 4, 4]
        ]),
        false
    );
    assert_eq!(
        Solution::is_rectangle_cover(vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![1, 3, 2, 4],
            vec![3, 2, 4, 4]
        ]),
        false
    );
    assert_eq!(
        Solution::is_rectangle_cover(vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![1, 3, 2, 4],
            vec![2, 2, 4, 4]
        ]),
        false
    );
}

// https://massivealgorithms.blogspot.com/2016/08/leetcode-391-perfect-rectangle.html
struct Solution {}
use std::collections::HashMap;
impl Solution {
    fn overlap(map: &mut HashMap<(i32, i32), i32>, vec2: (i32, i32), t: i32) -> bool {
        if let Some(m) = map.get_mut(&vec2) {
            if (*m & t) != 0 {
                return true;
            }
            *m = *m | t;
        } else {
            map.insert(vec2, t);
        }
        return false;
    }
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut map: HashMap<(i32, i32), i32> = HashMap::new();
        let mut lx = std::i32::MAX;
        let mut ly = std::i32::MAX;
        let mut rx = std::i32::MIN;
        let mut ry = std::i32::MIN;
        let mut sum = 0;
        for rec in rectangles {
            lx = std::cmp::min(lx, rec[0]);
            ly = std::cmp::min(ly, rec[1]);
            rx = std::cmp::max(rx, rec[2]);
            ry = std::cmp::max(ry, rec[3]);
            sum += (rec[2] - rec[0]) * (rec[3] - rec[1]);
            if Solution::overlap(&mut map, (rec[0], rec[1]), 1) {
                return false;
            }
            if Solution::overlap(&mut map, (rec[0], rec[3]), 2) {
                return false;
            }
            if Solution::overlap(&mut map, (rec[2], rec[1]), 4) {
                return false;
            }
            if Solution::overlap(&mut map, (rec[2], rec[3]), 8) {
                return false;
            }
        }

        let mut c = 0;
        for (_vec2, v) in map {
            if v != 15 && v != 12 && v != 10 && v != 9 && v != 6 && v != 5 && v != 3 {
                c += 1;
            }
        }

        return c == 4 && sum == (rx - lx) * (ry - ly);
    }
}
