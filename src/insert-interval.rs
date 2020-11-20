fn main() {
    assert_eq!(
        Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
        vec![[1, 5], [6, 9]]
    );
    assert_eq!(
        Solution::insert(
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16]
            ],
            vec![4, 8]
        ),
        vec![[1, 2], [3, 10], [12, 16]]
    );
    assert_eq!(Solution::insert(vec![], vec![5, 7]), vec![[5, 7]]);
    assert_eq!(Solution::insert(vec![vec![1, 5]], vec![2, 3]), vec![[1, 5]]);
    assert_eq!(Solution::insert(vec![vec![1, 5]], vec![2, 7]), vec![[1, 7]]);
    assert_eq!(Solution::insert(vec![vec![1, 5]], vec![5, 7]), vec![[1, 7]]);
    assert_eq!(
        Solution::insert(vec![vec![1, 5]], vec![6, 8]),
        vec![[1, 5], [6, 8]]
    );
    assert_eq!(Solution::insert(vec![vec![1, 5]], vec![0, 3]), vec![[0, 5]]);
    assert_eq!(
        Solution::insert(vec![vec![1, 5]], vec![0, 0]),
        vec![[0, 0], [1, 5]]
    );
    assert_eq!(Solution::insert(vec![vec![1, 5]], vec![0, 5]), vec![[0, 5]]);
    assert_eq!(Solution::insert(vec![vec![1, 5]], vec![0, 6]), vec![[0, 6]]);
    assert_eq!(
        Solution::insert(vec![vec![0, 5], vec![9, 12]], vec![7, 16]),
        vec![[0, 5], [7, 16]]
    );
    assert_eq!(
        Solution::insert(vec![vec![3, 5], vec![12, 15]], vec![6, 6]),
        vec![[3, 5], [6, 6], [12, 15]]
    );
}

pub struct Solution {}
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut data = intervals;
        data.push(new_interval);
        data.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..data.len() {
            if i > 0 {
                let idx = result.len() - 1;
                if result[idx][1] < data[i][0] {
                    result.push(data[i].clone());
                } else if data[i][1] < result[idx][0] {
                    if let Some(p) = result.pop() {
                        result.push(data[i].clone());
                        result.push(p);
                    }
                } else {
                    let start = if data[i].first() < result[idx].first() {
                        data[i][0]
                    } else {
                        result[idx][0]
                    };
                    let end = if data[i].last() > result[idx].last() {
                        data[i][1]
                    } else {
                        result[idx][1]
                    };
                    result.pop();
                    result.push(vec![start, end]);
                }
            } else {
                result.push(data[i].clone());
            }
        }
        return result;
    }
}
// impl Solution {
//     pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
//         if intervals.len() == 0 {
//             return vec![new_interval];
//         }
//         let mut is_interval = false;
//         let mut result: Vec<Vec<i32>> = Vec::new();
//         for i in 0..intervals.len() {
//             if i > 0 {
//                 let idx = result.len() - 1;
//                 if !is_interval && intervals[i][0] >  new_interval[0] && intervals[i][0] >  new_interval[1] {
//                     result.push(vec![new_interval[0], new_interval[1]]);
//                 }
//                 if result[idx][1] < intervals[i][0] {
//                     if new_interval[0] <= intervals[i][1] && new_interval[1] > intervals[i][1] {
//                         if intervals[i][0] > new_interval[0] {
//                             result.push(vec![new_interval[0], new_interval[1]]);
//                         } else {
//                             result.push(vec![intervals[i][0], new_interval[1]]);
//                         }
//                         is_interval = true;
//                     } else {
//                         result.push(intervals[i].clone());
//                     }
//                 } else if intervals[i][1] < result[idx][0] {
//                     if let Some(p) = result.pop() {
//                         if new_interval[0] <= intervals[i][1] && new_interval[1] > intervals[i][1] {
//                             result.push(vec![intervals[i][0], new_interval[1]]);
//                             is_interval = true;
//                         } else {
//                             result.push(intervals[i].clone());
//                         }
//                         result.push(p);
//                     }
//                 } else {
//                     let start = if intervals[i].first() < result[idx].first() {
//                         intervals[i][0]
//                     } else {
//                         result[idx][0]
//                     };
//                     let end = if intervals[i].last() > result[idx].last() {
//                         intervals[i][1]
//                     } else {
//                         result[idx][1]
//                     };
//                     result.pop();
//                     result.push(vec![start, end]);
//                 }
//             } else {
//                 // 1,5
//                 // 0,0
//                 // 0 <= 5 && 0 > 1
//                 // 0 <= 5 && 0 <= 5 && 1 > 0
//                 // 1 < 0

//                 // 1, 5
//                 // 0, 3
//                 // 0 < 5 && 3 <= 5 && 1 > 0

//                 // 1 <= 3 && 5 > 3

//                 // vec![vec![1, 5]], vec![0, 5])
//                 // 1 <= 5 && 5 >

//                 // [[1,5]] [0,6]
//                 // vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
//                 if new_interval[0] <= intervals[i][1] && new_interval[1] > intervals[i][1] {
//                     if intervals[i][0] > new_interval[0] {
//                         result.push(vec![new_interval[0], new_interval[1]]);
//                     } else {
//                         result.push(vec![intervals[i][0], new_interval[1]]);
//                     }
//                     is_interval = true;
//                 } else if intervals[i][0] <= new_interval[1] && intervals[i][0] > new_interval[0] {
//                     result.push(vec![new_interval[0], intervals[i][1]]);
//                 // } else if new_interval[0] == 0 && new_interval[1] == 0 && intervals[i][0] == 1 {
//                 //     result.push(vec![new_interval[0], intervals[i][1]]);
//                 is_interval = true;
//                 } else {
//                     result.push(intervals[i].clone());
//                 }
//             }
//         }
//         if let Some(interval) = result.last() {
//             if interval[1] < new_interval[0] {
//                 result.push(new_interval.clone());
//             }
//         }
//         if let Some(interval) = result.first() {
//             if interval[0] > new_interval[1] {
//                 result.insert(0, new_interval.clone());
//             }
//         }
//         return result;
//     }
// }
