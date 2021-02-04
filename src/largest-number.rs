fn main() {
    // assert_eq!(Solution::largest_number(vec![10, 2]), "210".to_string());
    // assert_eq!(
    //     Solution::largest_number(vec![3, 30, 34, 5, 9]),
    //     "9534330".to_string()
    // );
    // assert_eq!(Solution::largest_number(vec![1]), "1".to_string());
    // assert_eq!(Solution::largest_number(vec![10]), "10".to_string());
    // assert_eq!(
    //     Solution::largest_number(vec![432, 43243]),
    //     "43243432".to_string()
    // );
    assert_eq!(Solution::largest_number(vec![111311, 1113]), "1113111311".to_string());
}

pub struct Solution {}
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut m_nums = nums;
        m_nums.sort_by(|x, y| {
            if x.to_string().len() == y.to_string().len() {
                if x > y {
                    std::cmp::Ordering::Less
                } else if x == y {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Greater
                }
            } else if x.to_string().len() > y.to_string().len() {
                let yy = format!("{:0<width$}", y, width = x.to_string().len()).parse::<i32>().unwrap();
                return if x > &yy {
                    std::cmp::Ordering::Less
                } else if x == &yy {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Greater
                };
            } else {
                let xx = format!("{:0<width$}", x, width = y.to_string().len()).parse::<i32>().unwrap();
                return if &xx > y {
                    println!("debug {} {}", xx , y);
                    std::cmp::Ordering::Less
                } else if &xx == y {
                    std::cmp::Ordering::Equal
                } else {
                    // 111311
                    // 111300
                    println!("debug a {} {}", xx , y);
                    std::cmp::Ordering::Greater
                };
            }
        });

        // 1113111311
        // 1113111113
        println!("debug list {:?}", m_nums);

        return m_nums.iter().map(|x| x.to_string()).collect::<String>();
    }
}
