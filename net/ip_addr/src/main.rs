// use std::net::test::{sa4, tsa};
use std::net::Ipv4Addr;

mod test;
use test::{sa4, tsa};

// #[test]
fn to_socket_addr_socketaddr() {
    let a = sa4(Ipv4Addr::new(77, 88, 21, 11), 12345);
    assert_eq!(Ok(vec![a]), tsa(a));
}

fn main() {
    to_socket_addr_socketaddr();
}