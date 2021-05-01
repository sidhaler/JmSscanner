use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use std::io::ErrorKind;
#[derive(Debug)]
pub enum PortStat{
    Open,
    Closed,
    Filtered,
    NoHost,
    BrainDamage,
}

pub fn tcp_scan(add:SocketAddr, dur: Duration ) -> PortStat {
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