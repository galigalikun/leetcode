fn main() {
    assert_eq!(Solution::largest_time_from_digits(vec![1,2,3,4]), "23:41");
    assert_eq!(Solution::largest_time_from_digits(vec![5,5,5,5]), "");
}

struct Solution;
impl Solution {
    pub fn largest_time_from_digits(arr: Vec<i32>) -> String {
        let mut best_total_minutes: Option<i32> = None;

        for i in 0..4 {
            for j in 0..4 {
                if j == i {
                    continue;
                }
                for k in 0..4 {
                    if k == i || k == j {
                        continue;
                    }
                    for l in 0..4 {
                        if l == i || l == j || l == k {
                            continue;
                        }

                        let hour = arr[i] * 10 + arr[j];
                        let minute = arr[k] * 10 + arr[l];

                        if hour < 24 && minute < 60 {
                            let total_minutes = hour * 60 + minute;
                            best_total_minutes = match best_total_minutes {
                                Some(current_best) if current_best >= total_minutes => {
                                    Some(current_best)
                                }
                                _ => Some(total_minutes),
                            };
                        }
                    }
                }
            }
        }

        match best_total_minutes {
            Some(total_minutes) => {
                let hour = total_minutes / 60;
                let minute = total_minutes % 60;
                format!("{:02}:{:02}", hour, minute)
            }
            None => String::new(),
        }
    }
}
