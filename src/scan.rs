use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::time::Duration;

pub fn scan_ports(ip: String, port: String, scan_all: bool) {
    println!("Scanning ports for {}", ip);

    if scan_all {
        for port_number in 1..=65535 {
            let addr = format!("{}:{}", ip, port_number);
            match addr.parse::<SocketAddr>() {
                Ok(socket_addr) => {
                    if tcp_port_open(socket_addr) {
                        println!("Port {} is open (TCP)", port_number);
                    } else if udp_port_open(socket_addr) {
                        println!("Port {} is open (UDP)", port_number);
                    }
                }
                Err(e) => eprintln!("Error parsing address: {}", e),
            }
        }
    } else {
        let addr = format!("{}:{}", ip, port);
        match addr.parse::<SocketAddr>() {
            Ok(socket_addr) => {
                if tcp_port_open(socket_addr) {
                    println!("Port {} is open (TCP)", port);
                } else if udp_port_open(socket_addr) {
                    println!("Port {} is open (UDP)", port);
                } else {
                    println!("Port {} is closed", port);
                }
            }
            Err(e) => eprintln!("Error parsing address: {}", e),
        }
    }
}

fn tcp_port_open(addr: SocketAddr) -> bool {
    match TcpStream::connect_timeout(&addr, Duration::from_secs(0)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn udp_port_open(addr: SocketAddr) -> bool {
    match UdpSocket::bind(addr) {
        Ok(_) => true,
        Err(_) => false,
    }
}