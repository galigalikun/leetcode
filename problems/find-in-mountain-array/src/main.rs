fn main() {
    assert_eq!(
        Solution::find_in_mountain_array(3, &MountainArray::new(vec![1, 2, 3, 4, 5, 3, 1])),
        2
    );
    assert_eq!(
        Solution::find_in_mountain_array(3, &MountainArray::new(vec![0, 1, 2, 4, 2, 1])),
        -1
    );
}

struct Solution;

struct MountainArray {
    data: Vec<i32>,
}

impl MountainArray {
    fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    fn get(&self, index: i32) -> i32 {
        self.data[index as usize]
    }

    fn length(&self) -> i32 {
        self.data.len() as i32
    }
}

impl Solution {
    pub fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
        let n = mountain_arr.length();
        if n < 3 {
            return -1;
        }

        let mut left = 0;
        let mut right = n - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            let mid_val = mountain_arr.get(mid);
            let next_val = mountain_arr.get(mid + 1);
            if mid_val < next_val {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        let peak = left;
        let left_result = Solution::binary_search_ascending(target, mountain_arr, 0, peak);
        if left_result != -1 {
            return left_result;
        }

        Solution::binary_search_descending(target, mountain_arr, peak, n - 1)
    }

    fn binary_search_ascending(target: i32, mountain_arr: &MountainArray, mut lo: i32, mut hi: i32) -> i32 {
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let value = mountain_arr.get(mid);
            if value == target {
                return mid;
            }
            if value < target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        -1
    }

    fn binary_search_descending(target: i32, mountain_arr: &MountainArray, mut lo: i32, mut hi: i32) -> i32 {
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let value = mountain_arr.get(mid);
            if value == target {
                return mid;
            }
            if value > target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        -1
    }
}
