//noinspection RsExternalLinter
extern crate futures;
extern crate hyper;
extern crate tokio_core;

// IMP0RT PAKIETOW Z BIBLIOTEKI
mod websitesSearch;
use websitesSearch::jmsREQ;


fn main(){
    // test : ) 
    let s2  = "http://192.168.1.15/".to_string();
    jmsREQ(s2);

}

