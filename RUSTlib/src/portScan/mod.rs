extern crate tokio;
extern crate hyper;
extern crate futures;



use std::env;
use std::process;
use tokio::*;
use std::sync::{Arc, Mutex};
use std::thread;

fn jms(host: String, port: u16  ) -> bool{
    thread::sleep_ms(500);
    return false;
}




pub fn scanner(arg: String, mask: i32){
    /*    let mut args: Vec<String> = Vec::new();
        for arg in env::args_os() {
            args.push(arg.into_string().unwrap());
        }*/



    let ct_args: i32 = arg.len();

    if ct_args > 1 {
        println!("Port scanner \n
        https://github.com/sidhaler/jmsSCANNER ");
        process::exit(1);
    }


    let mut start_port = 1;
    let mut end_port= 1000;





    let ports: Vec<u16> = (start_port..end_port).rev().collect();

    let shared_ports = Arc::new(Mutex::new(ports));


    let mut threads = Vec::new();


    for i in (0..10){
        let port_clone = shared_ports.clone();

        let handle = thread::spawn(move || {
            //    println!("Spawned thread nr {}", i);
            loop {
                let mut port;
                {
                    let mut ports = port_clone.lock().unwrap();
                    port = ports.pop();
                }
                match port {
                    Some(port) => {
                        println!("{}", port);
                        if jms(host.clone(), port){
                            println!("Port {} OPEN", port)
                        }
                    },
                    None =>{
                        println!("Thread {} Closing", i);
                        break;
                    }
                }

            }
        });
        threads.push(handle);
    }
    for thread in threads{
        thread.join().unwrap();
    }

}
