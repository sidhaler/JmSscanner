use std::{thread, env};
use std::fmt;
use std::io::{ErrorKind, Result};
use std::net::{SocketAddr, TcpStream, IpAddr, Ipv4Addr};
use std::time::Duration;
use std::thread::Thread;
use std::result::Result::Ok;




#[tokio::main]
async fn main() -> std::io::Result<()>  {
    let args: Vec<String> = env::args().collect();

    let wrr = &args[1];
    let mut actddr = wrr.to_string();



    println!("Welcome in JmSscanner, source => https://github.com/sidhaler/jmsSCANNER \n\n\r\r");

    println!("SCANNING => {} || \n\n\r\r", actddr);




    let mut dur = Duration::from_millis(10);

    let mut c: i32 =  0;


    let target: Ipv4Addr = actddr.parse().expect("Argument Error");




    /// scanning for max range 1000
    for i in 1..1000{


        let mut portsat: i32 = 0;

        /// actual adress
        let mut add: SocketAddr = SocketAddr::new(IpAddr::V4(target), i);

        let t = thread::spawn(move ||{
            /// scanning ports
            match TcpStream::connect_timeout(&add, dur){
                Ok(stream) =>  println!("OPEN || PORT => {}/TCP || \n\r", i),
                Err(e) => match e.kind() {
                    // ErrorKind::TimedOut => println!("FILTERED/CLOSED || PORT => {}/TCP ||\n", i),
                    ErrorKind::TimedOut => portsat = 3,
                    ErrorKind::ConnectionRefused  => portsat = 1,
                    ErrorKind::ConnectionReset => portsat = 2,
                    _ => portsat = 4
                },
            };

            match portsat {
                1 => println!("CLOSED || PORT => {}/TCP || \n\r", i),
                2 => println!("FILTERED || PORT => {}/TCP || \n\r", i),
                3 => portsat = 0,
                4 => panic!("NO ROUTE TO HOST"),
                _ => portsat = 0
            };



            thread::sleep(Duration::from_millis(10));
        }
        );
        t.join().unwrap().clone();
    }
    Ok(())
}
