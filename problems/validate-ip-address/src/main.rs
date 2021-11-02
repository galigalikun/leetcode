fn main() {
    assert_eq!(
        Solution::valid_ip_address("172.16.254.1".to_string()),
        "IPv4"
    );
    assert_eq!(
        Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".to_string()),
        "IPv6"
    );
    assert_eq!(
        Solution::valid_ip_address("256.256.256.256".to_string()),
        "Neither"
    );
    assert_eq!(
        Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334:".to_string()),
        "Neither"
    );
    assert_eq!(
        Solution::valid_ip_address("1e1.4.5.6".to_string()),
        "Neither"
    );
    assert_eq!(
        Solution::valid_ip_address("01.01.01.01".to_string()),
        "Neither"
    );
    assert_eq!(
        Solution::valid_ip_address("2001:0db8:85a3:00000:0:8A2E:0370:7334".to_string()),
        "Neither"
    );
}

pub struct Solution {}
impl Solution {
    pub fn valid_ip_address(ip: String) -> String {
        let ipv4 = ip.split(".").collect::<Vec<_>>();
        let ipv4_c = ipv4.len();
        if ipv4_c > 1 {
            if ipv4_c != 4 {
                return "Neither".to_string();
            }
            for n in ipv4 {
                if n.len() > 1 && Some('0') == n.chars().nth(0) {
                    return "Neither".to_string();
                }
                if let Err(_err) = n.parse::<u8>() {
                    return "Neither".to_string();
                }
            }
            return "IPv4".to_string();
        }
        let ipv6 = ip.split(":").collect::<Vec<_>>();
        let ipv6_c = ipv6.len();
        if ipv6_c > 1 {
            if ipv6_c != 8 {
                return "Neither".to_string();
            }
            for h in ipv6 {
                if h.len() > 4 {
                    return "Neither".to_string();
                }
                if let Err(_err) = u16::from_str_radix(h, 16) {
                    return "Neither".to_string();
                }
            }
            return "IPv6".to_string();
        }
        return "Neither".to_string();
    }
}
