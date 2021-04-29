use std::thread;
use libc;
use std::fmt;
use std::io::{ErrorKind, Result};
use std::net::{SocketAddr, TcpStream, IpAddr, Ipv4Addr};
use std::time::Duration;
use std::thread::Thread;
use std::result::Result::Ok;


/// HALO MANY ALE PRZKESZONE


#[tokio::main]
async fn main() -> std::io::Result<()>  {

println!("WELCOME IN JmSscanner => https://github.com/sidhaler/jmsSCANNER ");


let mut dur = Duration::from_millis(10);

/// scanning for max range 1000
for i in 1..1000{
///    let mut addr = String::new();

///    addr = format!("{}:{}", net, e);


let mut portsat: i32 = 0;
    /// actual adress
    /// scanning only router in network 192.168.1.0/24
    let mut add: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192,168,1,1)), i);

       let t = thread::spawn(move ||{
        ///    println!("Thread: nr. => {}", e);
        ///
        /// scanning ports


            match TcpStream::connect_timeout(&add, dur){
                Ok(stream) =>  println!("OPEN || PORT => {}/TCP ||\n", i),
                Err(e) => match e.kind() {
                   // ErrorKind::TimedOut => println!("FILTERED/CLOSED || PORT => {}/TCP ||\n", i),
                    ErrorKind::TimedOut => portsat = 4,
                    ErrorKind::ConnectionRefused  => portsat = 1,
                    ErrorKind::ConnectionReset => portsat = 2,
                    _ => println!("No route to host")
                },
            };


            thread::sleep(Duration::from_millis(10));
}
        );
    t.join();
}
    Ok(())
}
