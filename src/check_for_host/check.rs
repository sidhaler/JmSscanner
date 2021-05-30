use fastping_rs::Pinger;
use fastping_rs::PingResult::*;

// 
#[derive(Debug, Clone)]
pub enum Addressstatus{
    Up,
    Down,
} 

//
fn look_for_host(witam: &str) -> bool {
    let (pinger, results ) = match Pinger::new(None, Some(56)) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Wild error occured us!, {}", e),
    };
    pinger.add_ipaddr(witam);
    pinger.run_pinger();
    loop {
        match results.recv() {
            Ok(results) => match results {
                Idle { addr } => {
                    return false;
                }
                Receive { addr, rtt} => {
                    return true;
                }
                
            },Err(_) => panic!("Wild Thread got stroke !")
        }
        
    }
}

pub fn sprawdzacz(siems: &str) -> Addressstatus { 
    match look_for_host(siems){ 
        true => return Addressstatus::Up,
        false => return Addressstatus::Down,
    }
}