// enum IpAddrKind {
//     V4,
//     V6,
// }
//
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
//
// static home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };
//
// static loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };

// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }

// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

fn main() {
    // let home = IpAddrKind::V4(String::from("127.0.0.1"));
    // let loopback = IpAddrKind::V6(String::from("::1"));
    // let home = IpAddrKind::V4(127, 0, 0, 1);
    // let loopback = IpAddrKind::V6(String::from("::1"));

    println!("Hello, world!");
}
