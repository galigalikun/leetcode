fn main() {
    assert_eq!(
        Solution::max_number_of_families(
            3,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 8],
                vec![2, 6],
                vec![3, 1],
                vec![3, 10]
            ]
        ),
        4
    );
    assert_eq!(
        Solution::max_number_of_families(2, vec![vec![2, 1], vec![1, 8], vec![2, 6]]),
        2
    );
    assert_eq!(
        Solution::max_number_of_families(4, vec![vec![4, 3], vec![1, 4], vec![4, 6], vec![1, 7]]),
        4
    );
}

struct Solution;
impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut reserved = vec![0; n as usize];
        for seat in reserved_seats {
            let (row, col) = (seat[0] as usize - 1, seat[1] as usize - 1);
            reserved[row] |= 1 << col;
        }
        let mut ans = 0;
        for row in reserved {
            if row & 0b0111111110 == 0 {
                ans += 2;
            } else if row & 0b0111100000 == 0 || row & 0b0000011110 == 0 || row & 0b0001111000 == 0
            {
                ans += 1;
            }
        }
        return ans;
    }
}
