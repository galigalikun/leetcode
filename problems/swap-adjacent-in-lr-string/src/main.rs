fn main() {
    assert_eq!(Solution::can_transform("RXXLRXRXL".to_string(), "XRLXXRRLX".to_string()), true);
    assert_eq!(Solution::can_transform("X".to_string(), "L".to_string()), false);
}

struct Solution{}
impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        for i in (0..start.len()).step_by(2) {
            if &start[i..i+2] == &end[i..i+2] {
                continue;
            }
            if match &start[i..i+2] {
                "XL" => "LX",
                "RX" => "XR",
                _ => "",
            } == &end[i..i+2] {
                continue;
            }
            return false;
        }
        return true;
    }
}
