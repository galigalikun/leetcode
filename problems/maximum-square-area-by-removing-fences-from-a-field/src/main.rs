fn main() {
    assert_eq!(Solution::maximize_square_area(4, 3, vec![2, 3], vec![2]), 4);
    assert_eq!(Solution::maximize_square_area(6, 7, vec![2], vec![4]), -1);
}

struct Solution;
impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut prev = 0;
        for &fence in h_fences.iter().chain(std::iter::once(&m)) {
            let height = fence - prev;
            prev = fence;
            let mut prev_v = 0;
            for &v_fence in v_fences.iter().chain(std::iter::once(&n)) {
                let width = v_fence - prev_v;
                prev_v = v_fence;
                let area = height * width;
                if area > max_area {
                    max_area = area;
                }
            }
        }
        if max_area == 0 {
            return -1;
        }

        return max_area;
    }
}
