use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use std::io::ErrorKind;

#[derive(Debug, Clone)]
pub enum PortStat{
    Open,
    Closed,
    Filtered,
    NoHost,
}
// This function response with enum PortStat
pub fn tcp_scan(add:SocketAddr, dur: Duration) -> PortStat {
    // opening tcp stream on port and matching output
    match TcpStream::connect_timeout(&add, dur){
        Ok(_stream) =>  return PortStat::Open,
        Err(e) => match e.kind() {
            ErrorKind::TimedOut => return PortStat::Filtered,
            ErrorKind::ConnectionRefused  => return PortStat::Closed,
            ErrorKind::ConnectionReset => return  PortStat::Filtered,
            ErrorKind::AddrNotAvailable => return PortStat::NoHost,
            _ => return PortStat::NoHost,
        },
    }

}
