use std::fs::File;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod http;
use http::{HttpRequest, HttpResp, HttpStatus, HttpVersion};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // listen to home IP at port 7878

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let mut file = File::open("../webserver/index.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // let resp = HttpResp::with_text_html(HttpVersion::V(1.1), 200, contents).to_string();
    println!(
        "Generated Request: {:#?}",
        HttpRequest::from_request(&http_request)
    );
    // for req in http_request {
    //     println!("Official Request: {}", req);
    // }
    // stream.write_all(resp.as_bytes()).unwrap();
}
