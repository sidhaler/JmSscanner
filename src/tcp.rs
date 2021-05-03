use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use std::io::ErrorKind;

#[derive(Debug, Clone)]
pub enum PortStat{
    Open,
    Closed,
    Filtered,
    NoHost,
    BrainDamage,
}
// This function response with enum PortStat
pub fn tcp_scan(add:SocketAddr, dur: Duration) -> PortStat {
    //
    let mut scanresponse: PortStat = PortStat::BrainDamage;
    match TcpStream::connect_timeout(&add, dur){
        Ok(_stream) =>  scanresponse = PortStat::Open,
        Err(e) => match e.kind() {
            ErrorKind::TimedOut => scanresponse = PortStat::Filtered,
            ErrorKind::ConnectionRefused  => scanresponse = PortStat::Closed,
            ErrorKind::ConnectionReset => scanresponse =  PortStat::Filtered,
            _ => scanresponse = PortStat::NoHost
        },
    }

    return scanresponse;
}