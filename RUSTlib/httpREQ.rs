//noinspection RsExternalLinter
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use reqwest::blocking;
use reqwest::Client;
use error_chain::error_chain;
use std::io::Read;
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
        println!("PORT 80 IS USED");

    } else {
        println!("PORT 80 IS NOT USED");
    }

    // let body = res.text().await?;
    Ok(())
}
