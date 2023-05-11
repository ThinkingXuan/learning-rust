enum IPAddrKind {
    IPV4,
    IPV6,
}

struct IPAddr {
    kind: IPAddrKind,
    address: String,
}

fn router(ip: IPAddrKind) {

}

fn main() {
    let four = IPAddrKind::IPV4;
    let six = IPAddrKind::IPV6;

    router(four);
    router(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    
}
