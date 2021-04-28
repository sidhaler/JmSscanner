use std::thread;
use libc;
use std::fmt;
use std::io::{ErrorKind, Result};
use std::net::{SocketAddr, TcpStream, IpAddr, Ipv4Addr};
use std::time::Duration;
use std::thread::Thread;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PortStats {
    Open,
    Closed,
    Filtered,
    BadHost,
}


#[tokio::main]
async fn main() -> std::io::Result<()>  {


let net: String = "192.168.1.1".to_string();
for e in 1..1000{
///    let mut addr = String::new();

///    addr = format!("{}:{}", net, e);

    let mut add = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192,168,1,1)), e);

        thread::spawn(move ||{
        ///    println!("Thread: nr. => {}", e);
            if let Ok(stream) = TcpStream::connect(add) {
                println!("PORT {} OPEN", e);
            }



            thread::sleep(Duration::from_millis(100));
        });
}
    Ok(())
}

