fn main() {
    assert_eq!(
        Solution::corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]], 5),
        vec![10, 55, 45, 25, 25]
    );
    assert_eq!(
        Solution::corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 2, 15]], 2),
        vec![10, 25]
    );
}

struct Solution;
impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut res = vec![0; n as usize];
        for booking in bookings {
            let (i, j, k) = (booking[0], booking[1], booking[2]);
            res[(i - 1) as usize] += k;
            if j < n {
                res[j as usize] -= k;
            }
        }
        for i in 1..n as usize {
            res[i] += res[i - 1];
        }
        return res;
    }
}
