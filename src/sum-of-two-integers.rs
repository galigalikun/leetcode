fn main() {
    // assert_eq!(Solution::get_sum(1, 2), 3);
    // assert_eq!(Solution::get_sum(2, 3), 5);
    // assert_eq!(Solution::get_sum(-1, 1), 0);
    assert_eq!(Solution::get_sum(-14, 16), 2);
}

pub struct Solution {}
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let (mut p, mut q) = if a >= 0 && b >= 0 {
            let mut result = vec![0; a as usize];
            result.append(&mut vec![0; b as usize]);
            (result, vec![0; 0])
        } else if a < 0 && b < 0 {
            let mut result = vec![0; (-1 * a) as usize];
            result.append(&mut vec![0; (-1 * b) as usize]);
            (vec![], result)
        } else if a >= 0 {
            (vec![0; a as usize], vec![0; (-1 * b) as usize])
        } else if b >= 0 {
            (vec![0; b as usize], vec![0; (-1 * a) as usize])
        } else {
            (vec![], vec![])
        };
        if p.len() > 0 && q.len() == 0 {
            return p.len() as i32;
        } else if p.len() >= q.len() {
            for _i in 0..q.len() {
                p.pop();
            }
            return p.len() as i32;
        } else if q.len() > p.len() {
            for _i in 0..p.len() {
                q.pop();
            }
            return -1 * q.len() as i32;
        }
        return 0;
    }
}
