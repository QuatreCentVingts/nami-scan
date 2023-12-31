use std::env;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [options]", args[0]);
        std::process::exit(1);
    }

    let mut ip = String::new();
    let mut port: Option<u16> = None;
    let mut scan_all = false;
    let mut show_graphic = false;
    let mut verbose = false;

    for i in 1..args.len() {
        match args[i].as_str() {
            "-i" | "--ip" => {
                ip = args[i + 1].clone();
            }
            "-p" | "--port" => {
                port = Some(args[i + 1].parse().expect("Invalid port number"));
            }
            "-a" | "--all" => {
                scan_all = true;
            }
            "-G" | "--graphic" => {
                show_graphic = true;
            }
            "-v" | "--verbose" => {
                verbose = true;
            }
            "-h" | "--help" => {
                println!("Help message here");
                std::process::exit(0);
            }
            _ => {}
        }
    }

}

