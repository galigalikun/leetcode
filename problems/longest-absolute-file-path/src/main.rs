fn main() {
    assert_eq!(
        Solution::length_longest_path("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext".to_string()),
        20
    );
    assert_eq!(Solution::length_longest_path("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext".to_string()), 32);
    assert_eq!(Solution::length_longest_path("a".to_string()), 0);
    assert_eq!(
        Solution::length_longest_path("file1.txt\nfile2.txt\nlongfile.txt".to_string()),
        12
    );

    assert_eq!(
        Solution::length_longest_path(
            "a\n\taa\n\t\taaa\n\t\t\tfile1.txt\naaaaaaaaaaaaaaaaaaaaa\n\tsth.png".to_string()
        ),
        29
    );
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut map = HashMap::new();
        let mut max = 0;
        for s in input.split("\n") {
            let deep = s.split("\t").count();
            let path = s.split("\t").last().unwrap();
            map.insert(deep, path);
            if path.split(".").count() > 1 {
                let mut num = 0;
                for (d, p) in map.clone() {
                    if deep > d {
                        num += p.len() + 1;
                    }
                }
                num += path.len();
                max = std::cmp::max(max, num);
            }
        }
        return max as i32;
    }
}
