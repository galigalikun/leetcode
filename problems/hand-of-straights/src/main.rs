use std::collections::BTreeMap;

fn main() {
    assert_eq!(Solution::is_n_straight_hand(vec![1,2,3,6,2,3,4,7,8], 3), true);
    assert_eq!(Solution::is_n_straight_hand(vec![1,2,3,4,5], 4), false);
}

struct Solution;
impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        if group_size <= 0 {
            return false;
        }

        let group_size = group_size as usize;
        if hand.len() % group_size != 0 {
            return false;
        }

        let mut counts = BTreeMap::<i32, i32>::new();
        for card in hand {
            *counts.entry(card).or_insert(0) += 1;
        }

        while let Some((&start, &need)) = counts.iter().next() {
            for card in start..start + group_size as i32 {
                let Some(&count) = counts.get(&card) else {
                    return false;
                };
                if count < need {
                    return false;
                }
            }

            let mut to_remove = Vec::new();
            for card in start..start + group_size as i32 {
                if let Some(count) = counts.get_mut(&card) {
                    *count -= need;
                    if *count == 0 {
                        to_remove.push(card);
                    }
                }
            }

            for card in to_remove {
                counts.remove(&card);
            }
        }

        true
    }
}
