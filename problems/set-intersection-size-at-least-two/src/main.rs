fn main() {
    assert_eq!(
        Solution::intersection_size_two(vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]]),
        3
    );
    assert_eq!(
        Solution::intersection_size_two(vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]]),
        5
    );
}

struct Solution {}
impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| {
            if a[1] == b[1] {
                b[0].cmp(&a[0])
            } else {
                a[1].cmp(&b[1])
            }
        });

        let mut answer = 0;
        let mut first = -1; // second-largest chosen value
        let mut second = -1; // largest chosen value

        for interval in intervals {
            let start = interval[0];
            let end = interval[1];

            if start > second {
                answer += 2;
                first = end - 1;
                second = end;
            } else if start > first {
                answer += 1;
                first = second;
                second = end;
            }
        }

        answer
    }
}
