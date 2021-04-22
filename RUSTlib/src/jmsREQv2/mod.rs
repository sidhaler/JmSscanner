extern crate tokio_core;


use reqwest;
use error_chain::*;
use std::io;


error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}



/*#[tokio::main]
pub async fn isWebsiteAvailbe(x: &String) -> bool {
    let mut odp = true;
    let res = reqwest::get(x).await?;
    let sta = res.status();




    odp
}


*/