fn main() {
    assert_eq!(
        Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
        1
    );
    assert_eq!(
        Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
        2
    );
    assert_eq!(
        Solution::find_content_children(vec![10, 9, 8, 7], vec![5, 6, 7, 8]),
        2
    );
    assert_eq!(
        Solution::find_content_children(vec![10, 9, 8, 7], vec![10, 9, 8, 7]),
        4
    );
}

struct Solution {}
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut ss = s;
        ss.sort();
        let mut gg = g;
        gg.sort();
        let mut result = 0;
        for n in ss {
            if let Some(p) = gg.iter().position(|&x| x <= n) {
                result += 1;
                gg.remove(p);
            }
        }
        return result;
    }
}
