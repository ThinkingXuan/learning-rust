fn main() {
    enum IPAddrKind {
        IPV4(String),
        IPV6(String),
    }

    let ipAddr = IPAddrKind::IPV4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));


    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

}