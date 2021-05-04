use std::thread;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};
use std::result::Result::Ok;
use structopt::StructOpt;
use many::tcp::{PortStat, tcp_scan};
use many::services::match_service;

#[derive(Debug, StructOpt)]
#[structopt()]
///
///
///
/// SCAN FOR OPEN PORTS ON TARGET
///
///
/// Only ipv4, and local network
///
///
/// Don't use it on localhost because it's kinda bugged.
///
///
/// Max value of port range is 65535
///
///
///
/// EXAMPLE:
/// jmsrequest 192.168.1.1 -p 1-1000                  it will check 1000 ports on host
struct Opt {
    /// target to scan
    //  halo many
    #[structopt(required = true)]
    target: String,



    /// port range to scan, max value 65535
    #[structopt(short = "p", long = "port", default_value = "1-1000")]
    port_range: String
}

#[tokio::main]
async fn main() -> std::io::Result<()>  {
    // idk
    let mut c: u16 = 0;
    //getting arguments for program
    let opt = Opt::from_args();
    let tar = opt.target;
    let ports: String = opt.port_range;
    // splitting max and min port
    let port_ranges: Vec<&str> = ports.split("-").collect();
    // port ranges
    let get_min = port_ranges[0].to_string();
    let get_max = port_ranges[1].to_string();
    // remember to read instructions !
    if get_max > "65535".to_string() || get_min > "65535".to_string() {
        eprintln!("MAX VALUE IS 65535\n\
        NAURA!...");
        std::process::exit(01);
    }
    // if u are not retard
    let min_port_num: u16 = get_min.parse().expect("Argument Error");
    let  mut max_port_num: u16 = get_max.parse().expect("Argument Error");
    // XD
    let w: u16 = max_port_num - min_port_num;
    // ^^
    if max_port_num == 65535 {
        max_port_num = max_port_num - 1;
    }
    // LOOKING FOR CRACKHEADS
    if min_port_num > max_port_num{
        eprintln!("MAX PORT MUST BE GRATER THAN MIN PORT !\n\
        NAURA!...");
        std::process::exit(05);
    }

    // no crackheads detected
    //parsing arguments
    let actddr: String = tar.parse().expect("Argument Error");
    // HALO MANY STAN => WYKESZONY
    println!(
        "\r\nWelcome in JmSscanner, use --help for help^^\n\r
        source => https://github.com/sidhaler/jmsSCANNER \n\n\r\r");
    println!(
        "SCANNING => {} || On TCP protocol || From {} to {} ports \n\n\r\r"
        , actddr, &get_min, &get_max);
    println!(
        "Resolving meaning of results↓\n\r*STATE* || *PORT NUMBER*/PROTOCOl => *SERVICE*\n\n\n\r\r\r");
    // timeout duration
    let timeout_dur = Duration::from_millis(1);
    // parsing arguments for use in loop
    let target: Ipv4Addr = actddr.parse().expect("Argument Error");
    // getting scan start time
    let start = Instant::now();
    println!("SCAN RESULTS  ↓\n\r");
    // port scan start
    for i in min_port_num-1..max_port_num+1 {
        // actual addr, and port for scan
        let add: SocketAddr = SocketAddr::new(IpAddr::V4(target), i);
        let t = thread::spawn( move ||{
            loop {
                // MATCHING RESULTS FROM STREAM
                match tcp_scan(*&add, timeout_dur) {
                    PortStat::Closed => println!("CLOSED  ||  {}/TCP   =>  {}", i, match_service(i)),
                    PortStat::Filtered => c = 1
                    //println!("FILTERED || PORT => {}/TCP || \n\r", i)
                    ,
                    PortStat::Open => println!("OPEN    ||  {}/TCP   =>  {}", i, match_service(i)),
                    PortStat::NoHost => panic!("Host Unreachable"),
                    _ => println!("No route to host")
                };
                break
            }
        });
        t.join().unwrap();
    };

    // printing diffrence between end of scan and start in time in secs
    println!("\nJmSscan done: in {} seconds scanned {}", start.elapsed().as_secs(), w);
    // after that program ends
    Ok(())
}
