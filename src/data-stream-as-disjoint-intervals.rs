struct SummaryRanges {
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {
    /** Initialize your data structure here. */
    fn new() -> Self {
        SummaryRanges { data: vec![] }
    }

    fn add_num(&mut self, val: i32) {
        self.data.push(val);
    }

    fn get_intervals(&mut self) -> Vec<Vec<i32>> {
        self.data.sort();
        let mut result = vec![];
        let mut start = -1;
        let mut end = -1;
        for i in &self.data {
            if start == -1 {
                start = *i;
                end = *i;
            } else if *i == end + 1 {
                end = *i;
            } else {
                result.push(vec![start, end]);
                start = *i;
                end = *i;
            }
        }
        if start != -1 {
            result.push(vec![start, end]);
        }
        return result;
    }
}

/**
 * Your SummaryRanges object will be instantiated and called as such:
 * let obj = SummaryRanges::new();
 * obj.add_num(val);
 * let ret_2: Vec<Vec<i32>> = obj.get_intervals();
 */
// https://dreamume.medium.com/leetcode-352-data-stream-as-disjoint-intervals-75e178107288
fn main() {
    let mut obj = SummaryRanges::new();
    obj.add_num(1);
    let ret_2: Vec<Vec<i32>> = obj.get_intervals();
    assert_eq!(ret_2, vec![[1, 1]]);
    obj.add_num(3);
    let ret_3: Vec<Vec<i32>> = obj.get_intervals();
    assert_eq!(ret_3, vec![[1, 1], [3, 3]]);
    obj.add_num(7);
    let ret_4: Vec<Vec<i32>> = obj.get_intervals();
    assert_eq!(ret_4, vec![[1, 1], [3, 3], [7, 7]]);
    obj.add_num(2);
    let ret_5: Vec<Vec<i32>> = obj.get_intervals();
    assert_eq!(ret_5, vec![[1, 3], [7, 7]]);
}
