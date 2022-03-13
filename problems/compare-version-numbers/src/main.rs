fn main() {
    assert_eq!(
        Solution::compare_version("1.01".to_string(), "1.001".to_string()),
        0
    );
    assert_eq!(
        Solution::compare_version("1.0".to_string(), "1.0.0".to_string()),
        0
    );
    assert_eq!(
        Solution::compare_version("0.1".to_string(), "1.1".to_string()),
        -1
    );
    assert_eq!(
        Solution::compare_version("1.0.1".to_string(), "1".to_string()),
        1
    );
    assert_eq!(
        Solution::compare_version("7.5.2.4".to_string(), "7.5.3".to_string()),
        -1
    );
}

struct Solution {}
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1: Vec<i32> = version1
            .split('.')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let v2: Vec<i32> = version2
            .split('.')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        if v1.len() > v2.len() {
            for i in 0..v1.len() {
                if i >= v2.len() {
                    if v1[i] > 0 {
                        return 1;
                    }
                } else {
                    if v1[i] > v2[i] {
                        return 1;
                    } else if v1[i] < v2[i] {
                        return -1;
                    }
                }
            }
        } else {
            for i in 0..v2.len() {
                if i >= v1.len() {
                    if v2[i] > 0 {
                        return -1;
                    }
                } else {
                    if v1[i] > v2[i] {
                        return 1;
                    } else if v1[i] < v2[i] {
                        return -1;
                    }
                }
            }
        }

        return 0;
    }
}
