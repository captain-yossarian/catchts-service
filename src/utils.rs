use std::net::{SocketAddr, ToSocketAddrs};

pub fn parse_ip(address: Option<&str>) -> Option<SocketAddr> {
    match address {
        Some(add) => match add.to_socket_addrs() {
            Ok(mut v) => v.next(),
            _ => None,
        },
        _ => None,
    }
}
