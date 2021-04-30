use std::{thread, env};
use std::fmt;
use std::io::{ErrorKind, Result};
use std::net::{SocketAddr, TcpStream, IpAddr, Ipv4Addr};
use std::time::Duration;
use std::thread::Thread;
use std::result::Result::Ok;


#[derive(Debug)]
pub enum PortStat{
    Open,
    Closed,
    Filtered,
    NoHost,
    BrainDamage,
}

fn Tcp_Scan( add:SocketAddr , dur: Duration ) -> PortStat {
    let mut scanres: PortStat = PortStat::BrainDamage;


    match TcpStream::connect_timeout(&add, dur){
        Ok(stream) =>  scanres = PortStat::Open,
        Err(e) => match e.kind() {
            // ErrorKind::TimedOut => println!("FILTERED/CLOSED || PORT => {}/TCP ||\n", i),
            ErrorKind::TimedOut => scanres = PortStat::Filtered,
            ErrorKind::ConnectionRefused  => scanres = PortStat::Closed,
            ErrorKind::ConnectionReset => scanres =  PortStat::Filtered,
            _ => scanres = PortStat::NoHost
        },
    }

    return scanres;
}



#[tokio::main]
async fn main() -> std::io::Result<()>  {
    let args: Vec<String> = env::args().collect();

    let wrr = &args[1];
    let mut portrange = &args[2];



    let mut actddr = wrr.to_string();


    println!("\nWelcome in JmSscanner, source => https://github.com/sidhaler/jmsSCANNER \n\n\r\r");

    println!("SCANNING => {} || On TCP protocol || With range of {} ports \n\n\r\r", actddr, portrange);
    println!("Resolving meaning of results↓\n\r*STATE* || *PORT NUMBER*/PROTOCOl \n\n\n\r\r\r");


    let mut dur = Duration::from_millis(5);

    let mut c: i32 =  0;


    let target: Ipv4Addr = actddr.parse().expect("Argument Error");
    let maxport: u16 = portrange.parse().expect("Argument Error");

    println!("RESULTS↓\n");
    /// scanning for max range 1000
    for i in 1..maxport+1{


        let mut smiec: i32 = 0;

        /// actual adress
        let mut add: SocketAddr = SocketAddr::new(IpAddr::V4(target), i);

        let t = thread::spawn(move ||{
            /// scanning ports



            match Tcp_Scan(*&add, dur) {
                PortStat::Closed => println!("CLOSED  ||  {}/TCP  \n\r", i),
                PortStat::Filtered => smiec = 1
                //println!("FILTERED || PORT => {}/TCP || \n\r", i)
                ,
                PortStat::Open => println!("OPEN    ||  {}/TCP  \n\r", i),
                PortStat::NoHost => panic!("Host Unreachable"),
                _ => println!("No route to host")
            };


            thread::sleep(Duration::from_millis(5));
        }
        );

        t.join().unwrap();
    }
    Ok(())
}
