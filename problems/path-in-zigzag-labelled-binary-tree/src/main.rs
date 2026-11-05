fn main() {
    assert_eq!(Solution::path_in_zig_zag_tree(14), vec![1, 3, 4, 14]);
    assert_eq!(Solution::path_in_zig_zag_tree(26), vec![1, 2, 6, 10, 26]);
}

struct Solution;
impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut current = label;
        let mut path = Vec::new();

        while current >= 1 {
            path.push(current);

            let mut level = 0;
            let mut node = current;
            while node > 0 {
                node >>= 1;
                level += 1;
            }

            let level_start = 1 << (level - 1);
            let level_end = (1 << level) - 1;
            let normal = level_start + level_end - current;
            current = normal / 2;
        }

        path.reverse();
        path
    }
}
