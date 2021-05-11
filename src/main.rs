use std::thread;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};
use std::result::Result::Ok;
use structopt::StructOpt;
use futures::executor::block_on;


use many::tcp::{PortStat, tcp_scan};
use many::services::match_service;
use many::website_script::{website_script_check, WebServer };
use std::thread::Thread;

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
///
///
///
/// Max value of port range is 65535
///
///
///
/// EXAMPLE:
///
///
/// jmsscanner 192.168.1.1 -p 1-1000                  it will check 1000 ports on host
/// jmsscanner 192.168.1.1 -p 1-65535 -w              it will check for max port range, and look for websites up
///
struct Opt {
    // target to scan
    /// target to scan, if u want to scan network type network/mask
    #[structopt(required = true)]
    target: String,


    //Option for website search on host
    /// scanning website
    #[structopt(short = "w", long = "website")]
    website_scan: bool,


    /// port range to scan, max value 65535
    #[structopt(short = "p", long = "ports", default_value = "1-1000")]
    port_range: String
}

#[tokio::main]
async fn main() -> std::io::Result<()>  {
    // options values
    let opt = Opt::from_args();
    let tar = opt.target;
    println!(
        "\r\nWelcome in JmSscanner, use --help for help^^\n\r
        source => https://github.com/sidhaler/jmsSCANNER \n\n\r\r");
    // idk
    let mut c: u16 = 0;

    let website_scan_script = opt.website_scan;

    //getting arguments for program
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
    let actddr12: String = actddr.clone();
    // HALO MANY STAN => WYKESZONY
    println!(
        "SCANNING => {} || On TCP protocol || From {} to {} ports \n\n\r\r"
        , actddr, &get_min, &get_max);
    println!(
        "Resolving meaning of results↓\n\r*STATE* || *PORT NUMBER*/PROTOCOl => *SERVICE*\n\n\n\r\r\r");
    // timeout duration
    let timeout_dur = Duration::from_millis(1);
    // parsing arguments for use in loop
    let target: Ipv4Addr = actddr.parse().expect("Argument Error");
    // performence
    let mut thread_pool = vec![];
    // getting scan start time
    let start = Instant::now();
    println!("SCAN RESULTS  ↓\n\r");
    // port scan start
    thread_pool.push(thread::spawn(move || {
        for i in min_port_num - 1..max_port_num + 1 {
        // actual addr, and port for scan
        let add: SocketAddr = SocketAddr::new(IpAddr::V4(target), i);
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
    }}));
    for kid in thread_pool {
        let _ = kid.join().unwrap();
    }

    // printing diffrence between end of scan and start in time in secs
    println!("\nJmSscan done: in {} seconds scanned {}", start.elapsed().as_secs(), w);

    // website test script
    if website_scan_script == true {
        match block_on(website_script_check(actddr12)) {
            Ok(WebServer::Up) => println!("\nWEBSITE RUNNING ON THIS HOST !"),
            _ => c = 1 ,
        }
    };
    // after that program ends
    return Ok(())
}
