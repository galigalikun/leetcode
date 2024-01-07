fn main() {
    assert_eq!(
        Solution::filter_restaurants(
            vec![
                vec![1, 4, 1, 40, 10],
                vec![2, 8, 0, 50, 5],
                vec![3, 8, 1, 30, 4],
                vec![4, 10, 0, 10, 3],
                vec![5, 1, 1, 15, 1]
            ],
            1,
            50,
            10
        ),
        vec![3, 1, 5]
    );
    assert_eq!(
        Solution::filter_restaurants(
            vec![
                vec![1, 4, 1, 40, 10],
                vec![2, 8, 0, 50, 5],
                vec![3, 8, 1, 30, 4],
                vec![4, 10, 0, 10, 3],
                vec![5, 1, 1, 15, 1]
            ],
            0,
            50,
            10
        ),
        vec![4, 3, 2, 1, 5]
    );
    assert_eq!(
        Solution::filter_restaurants(
            vec![
                vec![1, 4, 1, 40, 10],
                vec![2, 8, 0, 50, 5],
                vec![3, 8, 1, 30, 4],
                vec![4, 10, 0, 10, 3],
                vec![5, 1, 1, 15, 1]
            ],
            0,
            30,
            3
        ),
        vec![4, 5]
    );
}

struct Solution;
impl Solution {
    pub fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        let mut restaurants = restaurants;
        restaurants.sort_by(|a, b| {
            if a[1] == b[1] {
                b[0].cmp(&a[0])
            } else {
                b[1].cmp(&a[1])
            }
        });
        restaurants.retain(|r| {
            if vegan_friendly == 1 && r[2] == 0 {
                return false;
            }
            if r[3] > max_price {
                return false;
            }
            if r[4] > max_distance {
                return false;
            }
            true
        });
        return restaurants.iter().map(|r| r[0]).collect();
    }
}
