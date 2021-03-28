// The API isBadVersion is defined for you.
// isBadVersion(versions:i32)-> bool;
// to call it use self.isBadVersion(versions)
// https://programmerstart.com/article/68481594621/
impl Solution {
    /*
2126753390
1702766719
    */
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut start = 1;
        let mut end = n;
        while start + 1 < end {
            let mid = start + (end - start)/2;
            if self.isBadVersion(mid) {
                end = mid;
            } else {
                start = mid;
            }
        }

        if self.isBadVersion(start) {
            return start;
        }


		return end;
    }
}
