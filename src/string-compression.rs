fn main() {
    assert_eq!(
        Solution::compress(&mut vec!['a', 'a', 'b', 'b', 'c', 'c', 'c']),
        6
    );
    assert_eq!(Solution::compress(&mut vec!['a']), 1);
    assert_eq!(
        Solution::compress(&mut vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'
        ]),
        4
    );
    assert_eq!(
        Solution::compress(&mut vec!['a', 'a', 'a', 'b', 'b', 'a', 'a']),
        6
    );
}

pub struct Solution {}
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut prev = chars[0];
        let mut work = vec![chars[0]];
        let mut count = 1;
        for i in 1..chars.len() {
            if prev == chars[i] {
                count += 1;
            } else {
                if count > 1 {
                    let mut d = vec![];
                    loop {
                        d.push(std::char::from_digit(count % 10 as u32, 10).unwrap());
                        if count < 10 {
                            break;
                        }
                        count = count / 10;
                    }
                    work.append(&mut d.into_iter().rev().collect::<Vec<_>>());
                }
                count = 1;
                work.push(chars[i]);
            }
            prev = chars[i];
        }
        if count > 1 {
            let mut d = vec![];
            loop {
                d.push(std::char::from_digit(count % 10 as u32, 10).unwrap());
                if count < 10 {
                    break;
                }
                count = count / 10;
            }
            work.append(&mut d.into_iter().rev().collect::<Vec<_>>());
        }
        *chars = work;
        return chars.len() as i32;
    }
}
