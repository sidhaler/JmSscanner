//noinspection RsExternalLinter
extern crate futures;
extern crate hyper;
extern crate tokio_core;
// IMPORT LIB


//mod websiteScanner;

use reqwest;
use error_chain::*;
use std::io;


use std::net::*;
use futures::future::ok;




error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}



#[tokio::main]
pub async fn jmsREQ( x: &String) -> Result<()> {
    let res = reqwest::get(x).await?;
    //println!("Status: {}", res.status());
    let sta = res.status();
    let jms = x;
    if { sta == 200 || sta == 302 } {
        //     println!("ip: {}");

        println!("Website found on this ip = > {}", jms)
    } else {
        println!(" No websites found \n");
    }

    // let body = res.text().await?;
    Ok(())
}

