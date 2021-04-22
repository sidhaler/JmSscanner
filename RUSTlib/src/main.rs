extern crate tokio;
use std::net::{TcpStream, IpAddr, Ipv4Addr, Ipv6Addr, TcpListener};
use std::str::FromStr;
use std::time::Duration;
use std::io::prelude::*;

use std::io;

#[tokio::main]
pub async fn main(){
    let ip: String = "192.168.1.0".to_string();



    let mut def= String::new();

    match ip.as_str() {
        "192.168.1.0" => def = "192.168.1.".to_string(),
        "10.0.0.0" => def = "10.10.0.".to_string(),
        "172.16.0.0" => def = "172.16.0".to_string(),
        _ => println!("Bad ip"),
    };



    let mut i: i32 = 1;
    let mask: i32 = 254;

    //   let mut kys: IpAddr =  IpAddr::V4(Ipv4Addr::new(192,168,1,1) );

    for i in (i..mask+1){

        let x: String = i.to_string();
        let t: String = "80".to_string();
        // FULL ADDRESS/ wieśniacka metoda ; )
        let s = format!("{}{}:{}", def, x, t);

        let s: &str = &s;

        println!("checking => {}",s);
        match TcpStream::connect(s){

            Ok(mut stream) => {
                println!("host: {} UP \n", s.to_string());
                /*           let bb = b"HELLO";

                           stream.write(bb).unwrap();
                           let mut data = [0 as u8]*/

            },
            Err(e) => {
                let n = "No host found";
                println!("{} \n", n);
            }
        }
        println!("==============");
    }







}

