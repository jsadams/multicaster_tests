use std::net::{UdpSocket, SocketAddr, Ipv4Addr};
use std::thread;
use std::time::{Duration, SystemTime};

const MULTICAST_ADDR: &'static str = "224.0.0.50:55583";
const INTERVAL_SECONDS: u64 = 1;

fn main() {
    // Parse multicast address
    let multicast_addr: SocketAddr = MULTICAST_ADDR.parse().expect("Invalid multicast address");

    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");

    // Set socket TTL (Time-To-Live) to enable multicast on the local network
    socket.set_multicast_ttl_v4(1).expect("Failed to set multicast TTL");

    // Main loop to send messages every second
    let mut counter = 0;
    loop {
        let message = format!("Hello world {}", counter);
        let _ = socket.send_to(message.as_bytes(), &multicast_addr);

        println!("Sent: {}", message);

        counter += 1;

        thread::sleep(Duration::from_secs(INTERVAL_SECONDS));
    }
}
