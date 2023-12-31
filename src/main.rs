mod scan;

use std::{env, io};
use std::io::Write;
use std::net::SocketAddr;
use crate::scan::scan_ports;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut scan_all = false;

    if args.len() < 2 {
        loop {
            let mut cmd = String::new();
            println!("Enter a command, example:
            <scan> <ip>:<port/all>
            <ping> <ip>
            <conn> <processus/all>");

            print!("\nEnter your cmd: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut cmd).expect("An error occurred while reading your command.");

            let cmd = cmd.trim();

            if cmd.is_empty() {
                println!("Can you specify a command?");
            }

            let cmd_eq: Vec<&str> = cmd.split_whitespace().collect();

            if cmd_eq.get(0) == Some(&"scan") {
                if let Some(ip_port) = cmd_eq.get(1) {
                    let ip_port_parts: Vec<&str> = ip_port.split(':').collect();

                    if let (Some(ip), Some(port)) = (ip_port_parts.get(0), ip_port_parts.get(1)) {
                        let ip_port_str = format!("{}:{}", ip, port);
                        let socket_addr: SocketAddr = ip_port_str.parse().expect("Invalid IP:Port format");

                        if port.eq_ignore_ascii_case("all") {
                            scan_all = true;
                        }

                        scan_ports(socket_addr.ip().to_string(), socket_addr.port().to_string(), scan_all);
                    } else {
                        println!("Invalid format for 'scan' command");
                    }
                } else {
                    println!("Invalid format for 'scan' command");
                }
            } else {
                println!("Unknown command or invalid format.");
            }
            println!("\n");
        }
    } else {

    }
}
