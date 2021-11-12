pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(u16, u16, u16, u16, u16, u16, u16, u16),
}

// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for IpAddr {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IpAddr::V4(ip1, ip2, ip3, ip4) => write!(f, "{}.{}.{}.{}", ip1, ip2, ip3, ip4),
            IpAddr::V6(ip1, ip2, ip3, ip4, ip5, ip6, ip7, ip8) => write!(
                f,
                "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
                ip1, ip2, ip3, ip4, ip5, ip6, ip7, ip8
            ),
        }
    }
}
