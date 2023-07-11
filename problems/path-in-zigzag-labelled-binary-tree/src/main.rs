fn main() {
    assert_eq!(Solution::path_in_zig_zag_tree(14), vec![1, 3, 4, 14]);
    assert_eq!(Solution::path_in_zig_zag_tree(26), vec![1, 2, 6, 10, 26]);
}

struct Solution;
impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut label = label;
        let mut level = 0;
        while label > 0 {
            label /= 2;
            level += 1;
        }
        let mut res = vec![0; level];
        let mut level = level - 1;
        while label > 0 {
            res[level] = label;
            label /= 2;
            level -= 1;
        }
        return res;
    }
}
