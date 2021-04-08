//noinspection RsExternalLinter
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate net;
// IMPORT LIB




use reqwest;
use error_chain::error_chain;
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
pub async fn jmsREQ(x: String) -> Result<()> {
    let res = reqwest::get(x).await?;
    //println!("Status: {}", res.status());
    let sta = res.status();
    if  {sta == 200 || sta == 302 }{
        println!("ip: {} -> PORT 80 otwarty", x);

    } else {
        println!(" \n ");
    }

    // let body = res.text().await?;
    Ok(())
}

