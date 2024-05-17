

//use std::net::{UdpSocket, SocketAddr, IpAddr, SocketAddrV4, Ipv4Addr};
use std::net::{UdpSocket, SocketAddr, IpAddr};

use std::time::Duration;

const MULTICAST_ADDR: &'static str = "224.0.0.50:55583";
const BIND_ADDR: &'static str = "0.0.0.0:0"; // Any available port on any interface
const TIMEOUT_SECONDS: u64 = 5;

fn main() {
    // Parse multicast address and bind address
    let multicast_addr: SocketAddr = MULTICAST_ADDR.parse().expect("Invalid multicast address");
    let bind_addr: SocketAddr = BIND_ADDR.parse().expect("Invalid bind address");

    // Create a UDP socket
    let socket = UdpSocket::bind(bind_addr).expect("Failed to bind socket");

    // Parse multicast address
    let multicast_addr: SocketAddr = MULTICAST_ADDR.parse().expect("Invalid multicast address");
    let bind_addr: SocketAddr=BIND_ADDR.parse().expect("Invalid bind addr");

    // Create a UDP socket
    //let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");

    // Join the multicast group
    //let multicast_ip:IpAddr = multicast_addr.ip();
    //let bind_ip :IpAddr = bind_addr.ip();
    // if let (IpAddr::V4(multicast_ip), IpAddr::V4(bind_ip)) = (multicast_ip, bind_ip) {
    //     socket.join_multicast_v4(&multicast_ip, &bind_ip).expect("Failed to join multicast group");
    // } else {
    //     panic!("Multicast address or bind address is not IPv4");
    // }

    // // Join the multicast group
    // let multicast_ip = if let IpAddr::V4(ipv4) = multicast_addr.ip() {
    //     ipv4
    // } else {
    //     panic!("Multicast address is not IPv4");
    // };
    //
    // let bind_ip = if let IpAddr::V4(ipv4) = bind_addr.ip() {
    //     ipv4
    // } else {
    //     panic!("Bind address is not IPv4");
    // };

    socket.join_multicast_v4(&multicast_addr, &bind_addr).expect("Failed to join multicast group");


    //let multicast_ip: IpvAddr::V4 = multicast_addr.ip().to_ipv4().expect("Multicast address is not IPv4");
    // let bind_ip: Ipv4Addr = bind_addr.ip().to_ipv4().expect("Bind address is not IPv4");
    // socket.join_multicast_v4(&multicast_ip, &bind_ip).expect("Failed to join multicast group");

    // Set socket timeout
    socket.set_read_timeout(Some(Duration::from_secs(TIMEOUT_SECONDS))).expect("Failed to set socket timeout");

    println!("Waiting for data from {}", multicast_addr);

    // Main loop to receive and print data
    let mut buf = [0u8; 255];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, _)) => {
                let message = String::from_utf8_lossy(&buf[..size]);
                println!("{}", message);
            }
            Err(_) => {
                println!("Timeout reached. No data received within {} seconds.", TIMEOUT_SECONDS);
                //break;
            }
        }
    }
}

// use std::net::{UdpSocket, SocketAddr};
// use std::time::Duration;
// use std::convert::TryInto;
// use std::io::Error;
//
//
// fn main()  {
//     // Define multicast address, bind address, and port
//     let multicast_addr: SocketAddr = "224.0.0.50:55583".parse()?;
//     let bind_addr: SocketAddr = "0.0.0.0:0".parse()?; // Listen on any interface
//
//     // Create a UDP socket
//     let socket = UdpSocket::bind(bind_addr)?;
//
//     // Set multicast TTL (Time To Live) to 1 (local network only)
//     socket.set_ttl(1)?;
//
//     let mut count = 0;
//     loop {
//         // Create message string
//         let msg = format!("Hello {}", count);
//
//         // Print message to be sent
//         println!("Sending msg: {}", msg);
//
//         // Send message over multicast socket
//         socket.send_to(msg.as_bytes(), multicast_addr)?;
//
//         // Sleep for 0.3 seconds
//         std::thread::sleep(Duration::from_millis(300));
//
//         count += 1;
//     }
// }
