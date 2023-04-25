fn main() {
    assert_eq!(Solution::min_domino_rotations(vec![2,1,2,4,2,2], vec![5,2,6,2,3,2]), 2);
    assert_eq!(Solution::min_domino_rotations(vec![3,5,1,2,3], vec![3,6,3,3,4]), -1);
}

struct Solution;
impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut top = vec![0; 7];
        let mut bottom = vec![0; 7];
        let mut same = vec![0; 7];
        for i in 0..tops.len() {
            let t = tops[i];
            let b = bottoms[i];
            top[t as usize] += 1;
            bottom[b as usize] += 1;
            if t == b {
                same[t as usize] += 1;
            }
        }
        for i in 1..7 {
            if top[i] + bottom[i] - same[i] == tops.len() as i32 {
                return tops.len() as i32 - top[i].max(bottom[i]);
            }
        }
        println!("debug {:?} {:?}, {:?}", top, bottom, same);
        return 0;
    }
}
