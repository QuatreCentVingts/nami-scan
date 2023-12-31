use std::net::TcpStream;
use std::time::Duration;

pub fn scan_ports(ip: String, port: String, scan_all: bool) {
    println!("Scanning ports for {}", ip);

    if scan_all {
        for port in 1..=65535 {
            let addr = format!("{ip}:{port}");
            match TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_secs(0)) {
                Ok(_) => println!("Port {} is open", port),
                Err(_) => println!("port {} is closed", port),
            }
        }
    }else {
        let addr = format!("{ip}:{port}");
        match TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_secs(0)) {
            Ok(_) => println!("Port {} is open", port),
            Err(_) => println!("port {} is closed", port),
        }
    }
}
