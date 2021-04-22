use std::net::TcpStream;

fn jms323232(host: String) -> bool{


return true;
}

pub fn netscan(network: String, arg: i32){



    // get num of max available hosts in network
    let mut jmsMASK: i32 = 0;
    match arg {

        // set mask range to 30
        24 => jmsMASK = 254,
        25 => jmsMASK = 126,
        26 => jmsMASK = 62,
        _ => println!("No mask specified"),
    }




    let stream = TcpStream::new(host);


}