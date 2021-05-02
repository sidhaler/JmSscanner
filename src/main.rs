use std::{thread, env};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};
use many::tcp::{PortStat, tcp_scan};
use std::result::Result::Ok;


#[tokio::main]
async fn main() -> std::io::Result<()>  {
    // program arguments
    let args: Vec<String> = env::args().collect();
    // target
    let wrr = &args[1];
    // range of ports
    let portrange= &args[2];

    //converting target to string from &string for later use
    let actddr = wrr.to_string();

    println!(
        "\r\nWelcome in JmSscanner, source => https://github.com/sidhaler/jmsSCANNER \n\n\r\r");
    println!(
        "SCANNING => {} || On TCP protocol || With range of {} ports \n\n\r\r"
             , actddr, &portrange);
    println!(
        "Resolving meaning of results↓\n\r*STATE* || *PORT NUMBER*/PROTOCOl \n\n\n\r\r\r");
    // timeout duration
    let timeout_dur = Duration::from_millis(2);

    // idk
    let mut c: i32 =  0;

    // parsing arguments for use
    let target: Ipv4Addr = actddr.parse().expect("Argument Error");
    let maxport: u16 = portrange.parse().expect("Argument Error");

    // getting date
    let mut start = Instant::now();

    println!("RESULTS OF SCANN ↓\n");
    for i in 1..maxport+1{
        // actual addr
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
            thread::sleep(Duration::from_millis(2));

        }

        );
        t.join().unwrap();
    };
    println!("JmSscan done: in {} seconds scanned {} ports", start.elapsed().as_secs(), &maxport);
    Ok(())
}
