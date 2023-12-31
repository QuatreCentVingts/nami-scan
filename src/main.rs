mod scan;

use std::{env, io};
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use crate::scan::scan_ports;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ip = String::new();
    let mut scan_all = false;
    let mut show_graphic = false;
    let mut verbose = false;


    if args.len() < 2 {
        let mut cmd = String::new();
        print!("Enter a command, example:\n\tscan <ip> <port/all> <tcp/udp/all>\n\tping <ip>\n\nEnter your cmd: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut cmd).expect("An error occurred while reading your command.");

        let cmd = cmd.trim();

        if cmd.is_empty() {
            println!("Can you specify a command?");
        }

        let cmd_eq: Vec<&str> = cmd.split_whitespace().collect();

        if cmd_eq.get(0) == Some(&"scan") {
            if let (Some(ip), Some(port)) = (cmd_eq.get(1), cmd_eq.get(2)) {
                let ip = ip.to_string();
                let port = port.to_string();

                if port.eq_ignore_ascii_case("All") {
                    scan_all = true;
                    scan_ports(ip, port, scan_all);
                }
            }
            else {
                println!("Invalid format for 'scan' command");
            }
        }


        else {
            println!("Unknown command or invalid format.");
        }
    }
}

