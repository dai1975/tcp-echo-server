#[macro_use]
extern crate clap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(Debug, Clone, Copy)]
struct Option {
    port: u16,
    datetime: bool,
    peer_addr: bool,
}
impl Option {
    fn new() -> Self {
        Option {
            port: 0,
            datetime: false,
            peer_addr: false,
        }
    }
}

fn handle_client(mut stream: TcpStream, opt: &Option) {
    if opt.datetime {
        let s = format!("datetime: {}\n", chrono::offset::Utc::now());
        stream.write(s.as_bytes()).unwrap();
    }
    if opt.peer_addr {
        let s = format!("peer_addr: {}\n", &stream.peer_addr().unwrap());
        stream.write(s.as_bytes()).unwrap();
    }
    let mut data = [0u8; 128];
    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(e) => {
            println!("read error: {}", e);
            false
        }
    } {}
}

fn parse_args() -> Option {
    use clap::{App, AppSettings};
    let mut opt = Option::new();
    let matches = App::new(crate_name!())
        .args_from_usage(
            "[DATETIME] --datetime 'return server\'s datetime'
             [PEER_ADDR] --peer_addr 'return peer address'
             <PORT> 'port number to bind'",
        )
        .setting(AppSettings::DeriveDisplayOrder)
        .get_matches();
    opt.port = value_t_or_exit!(matches, "PORT", u16);
    opt.datetime = matches.is_present("DATETIME");
    opt.peer_addr = matches.is_present("PEER_ADDR");
    opt
}

fn main() {
    let opt = parse_args();
    println!("run {} with {:?}", crate_name!(), &opt);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", opt.port)).unwrap();
    println!("bind :{}", opt.port);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream, &opt);
                });
            }
            Err(e) => {
                println!("Err: {}", e);
            }
        }
    }
    drop(listener);
}
