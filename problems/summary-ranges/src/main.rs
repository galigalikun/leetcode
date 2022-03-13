fn main() {
    assert_eq!(
        Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
        vec!["0->2", "4->5", "7"]
    );
    assert_eq!(
        Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
        vec!["0", "2->4", "6", "8->9"]
    );
    // assert_eq!(Solution::summary_ranges(vec![]), vec![]);
}

struct Solution {}
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut prev: Option<i32> = None;
        let mut last: Option<i32> = None;
        for n in nums {
            last = None;
            if let Some(p) = prev {
                if n != p + 1 {
                    if let Some(l) = result.last_mut() {
                        if l.parse::<i32>().unwrap() != p {
                            *l = format!("{}->{}", l, p);
                        }
                    }
                    result.push(n.to_string());
                } else {
                    last = Some(n);
                }
            } else {
                result.push(n.to_string());
            }
            prev = Some(n);
        }
        if let Some(l) = last {
            if let Some(ls) = result.last_mut() {
                *ls = format!("{}->{}", ls, l);
            }
        }
        return result;
    }
}
