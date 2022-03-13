fn main() {
    assert_eq!(
        Solution::add_operators("123".to_string(), 6),
        vec!["1+2+3", "1*2*3"]
    );

    assert_eq!(
        Solution::add_operators("232".to_string(), 8),
        vec!["2+3*2", "2*3+2"]
    );

    assert_eq!(
        Solution::add_operators("105".to_string(), 5),
        vec!["1*0+5", "10-5"]
    );

    assert_eq!(
        Solution::add_operators("00".to_string(), 0),
        vec!["0+0", "0-0", "0*0"]
    );

    // assert_eq!(
    //     Solution::add_operators("3456237490".to_string(), 9191),
    //     vec![]
    // );
}

struct Solution {}
impl Solution {
    fn helper(
        result: &mut Vec<String>,
        sb: &mut String,
        num: String,
        pos: usize,
        target: i64,
        prev: i64,
        multi: i64,
    ) {
        if pos == num.len() {
            if target == prev {
                result.push(sb.to_string());
            }
            return;
        }

        for i in pos..num.len() {
            if num.chars().nth(pos) == Some('0') && i != pos {
                break;
            }
            let n = num[pos..i + 1].parse::<i64>().unwrap();
            let len = sb.len();

            if pos == 0 {
                sb.push_str(&num[pos..i + 1]);
                Solution::helper(result, sb, num.clone(), i + 1, target, n, n);
                *sb = sb[0..len].to_string();
            } else {
                sb.push_str("+");
                sb.push_str(&num[pos..i + 1]);
                Solution::helper(result, sb, num.clone(), i + 1, target, prev + n, n);
                *sb = sb[0..len].to_string();
                sb.push_str("-");
                sb.push_str(&num[pos..i + 1]);
                Solution::helper(result, sb, num.clone(), i + 1, target, prev - n, -n);
                *sb = sb[0..len].to_string();
                sb.push_str("*");
                sb.push_str(&num[pos..i + 1]);
                Solution::helper(
                    result,
                    sb,
                    num.clone(),
                    i + 1,
                    target,
                    prev - multi + multi * n,
                    multi * n,
                );
                *sb = sb[0..len].to_string();
            }
        }
    }
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut result = vec![];
        Solution::helper(&mut result, &mut String::new(), num, 0, target as i64, 0, 0);
        return result;
    }
}
