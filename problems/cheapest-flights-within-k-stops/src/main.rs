fn main() {
    assert_eq!(
        Solution::find_cheapest_price(
            4,
            vec![
                vec![0, 1, 100],
                vec![1, 2, 100],
                vec![2, 0, 100],
                vec![1, 3, 600],
                vec![2, 3, 200],
            ],
            2,
            3,
            1,
        ),
        200
    );
    assert_eq!(
        Solution::find_cheapest_price(
            3,
            vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
            0,
            2,
            1,
        ),
        200
    );
    assert_eq!(
        Solution::find_cheapest_price(
            3,
            vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
            0,
            2,
            0,
        ),
        500
    );
}

struct Solution{}
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let city_count = n as usize;
        let src_index = src as usize;
        let dst_index = dst as usize;
        let edge_limit = (k + 1) as usize;
        let inf = i32::MAX / 4;

        let mut best = vec![inf; city_count];
        best[src_index] = 0;

        for _ in 0..edge_limit {
            let mut next = best.clone();

            for flight in &flights {
                if flight.len() != 3 {
                    continue;
                }

                let from = flight[0] as usize;
                let to = flight[1] as usize;
                let price = flight[2];

                if from >= city_count || to >= city_count || best[from] == inf {
                    continue;
                }

                let candidate = best[from] + price;
                if candidate < next[to] {
                    next[to] = candidate;
                }
            }

            best = next;
        }

        if best[dst_index] == inf {
            -1
        } else {
            best[dst_index]
        }
    }
}
