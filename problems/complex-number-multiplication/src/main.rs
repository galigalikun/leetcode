fn main() {
    assert_eq!(
        Solution::complex_number_multiply("1+1i".to_string(), "1+1i".to_string()),
        "0+2i"
    );
    assert_eq!(
        Solution::complex_number_multiply("1+-1i".to_string(), "1+-1i".to_string()),
        "0+-2i"
    );
}

struct Solution {}
impl Solution {
    fn parser(num: String) -> (i32, i32) {
        let (real_str, imag_with_i) = num
            .split_once('+')
            .expect("invalid complex number format");
        let imag_str = imag_with_i.trim_end_matches('i');

        let real = real_str.parse::<i32>().expect("invalid real part");
        let imaginary = imag_str.parse::<i32>().expect("invalid imaginary part");

        return (real, imaginary);
    }
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let c1 = Solution::parser(num1);
        let c2 = Solution::parser(num2);
        return format!(
            "{}+{}i",
            c1.0 * c2.0 - c1.1 * c2.1,
            c1.0 * c2.1 + c1.1 * c2.0
        );
    }
}
