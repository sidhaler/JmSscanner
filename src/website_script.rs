use reqwest::*;
use error_chain::*;
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
#[derive(Debug, Clone)]
pub enum WebServer{
    Up,
    Down,
}
fn response_status(res: Response) -> Result<WebServer> {
    match res.error_for_status() {
        Ok(_res) => return Ok(WebServer::Up),
        Err(e) => return Ok(WebServer::Down),
    }
}
pub async fn website_script_check(host: String ) -> Result<WebServer>{


    let url = format!("http://{}/", host);

    let JmSrequest = reqwest::get(url).await?;

    match response_status(JmSrequest) {
       Ok(_res) =>  return Ok(WebServer::Up),
       Err(err) =>  return Ok(WebServer::Down),
    }
}
