use std::thread;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};
use many::tcp::{PortStat, tcp_scan};
use std::result::Result::Ok;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt()]
/// SCAN FOR OPEN PORTS ON TARGET
/// Only ipv4, and local network
/// Don't use it on localhost because it's kinda bugged.
///
/// EXAMPLE:
/// jmsrequest 192.168.1.1 -p 1000                  it will check 1000 ports on host
struct Opt {
    /// target to scan
    //  halo many
    #[structopt(required = true)]
    target: String,



    /// port range to scan
    #[structopt(short = "p", long = "port", default_value = "1000")]
    port_range: String
}

#[tokio::main]
async fn main() -> std::io::Result<()>  {
    let mut c: u8 = 0;
    //getting options
    let opt = Opt::from_args();
    let tar = opt.target;
    let ports = opt.port_range;
    //parsing arguments
    let portrange: String = ports.parse().expect("Argument Error");
    let actddr: String = tar.parse().expect("Argument Error");


    // HALO MANY STAN => WYKESZONY
    println!(
        "\r\nWelcome in JmSscanner, use --help to see options\n\r
        source => https://github.com/sidhaler/jmsSCANNER \n\n\r\r");
    println!(
        "SCANNING => {} || On TCP protocol || With range of {} ports \n\n\r\r"
        , actddr, &portrange);
    println!(
        "Resolving meaning of results↓\n\r*STATE* || *PORT NUMBER*/PROTOCOl \n\n\n\r\r\r");
    // timeout duration
    let timeout_dur = Duration::from_millis(1);


    // parsing arguments for use in loop
    let target: Ipv4Addr = actddr.parse().expect("Argument Error");
    let maxport: u16 = portrange.parse().expect("Argument Error");

    // getting scan start time
    let start = Instant::now();
    println!("SCAN  ↓\n\r");
    // startring scan
    for i in 1..maxport+1{
        // actual addr, and port
        let add: SocketAddr = SocketAddr::new(IpAddr::V4(target), i);
        let t = thread::spawn(move ||{
            loop {
                // MATCHING RESULTS FROM STREAM
                match tcp_scan(*&add, timeout_dur) {
                    PortStat::Closed => println!("CLOSED  ||  {}/TCP  \n\r", i),
                    PortStat::Filtered => c = 1
                    //println!("FILTERED || PORT => {}/TCP || \n\r", i)
                    ,
                    PortStat::Open => println!("OPEN    ||  {}/TCP  \n\r", i),
                    PortStat::NoHost => panic!("Host Unreachable"),
                    _ => println!("No route to host")
                };

                return
            }
        }
        );
        t.join().unwrap();
    };
    // printing diffrence between start of scan and end of scan in secs
    println!("JmSscan done: in {} seconds scanned {} ports", start.elapsed().as_secs(), &maxport);
    // after that program is ending
    Ok(())
}
