use std::fs::{self, File};
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod http;
use http::{HttpMethod, HttpRequest, HttpResp, HttpStatus, HttpVersion};

fn main() {
    let ip = "127.0.0.1:7878";
    let listener = TcpListener::bind(ip).unwrap(); // listen to home IP at port 7878

    println!("Opened connection at: http://{}", ip);

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

    let req = HttpRequest::from_request(&http_request);

    let page_404 =
        fs::read_to_string("../webserver/404.html").expect("Expect 404 page file to exist");
    let resp_404 = HttpResp::with_text_html(HttpVersion::V(1.1), 404, page_404);
    if let Ok(request) = req {
        if *request.method() == HttpMethod::GET {
            let response = serve_html(request.uri()).unwrap_or_else(|_| resp_404);
            stream.write_all(response.to_string().as_bytes()).unwrap();
        };
    }

    fn serve_html(uri: &str) -> Result<HttpResp, std::io::Error> {
        let contents = match uri {
            "/" => {
                // on '/' serve the default page
                fs::read_to_string("../webserver/index.html")?
            }
            _ => {
                let path = if uri.ends_with(".html") {
                    format!("../webserver/{}", uri)
                } else {
                    format!("../webserver/{}.html", uri)
                };

                fs::read_to_string(path)?
            }
        };
        Ok(HttpResp::with_text_html(HttpVersion::V(1.1), 200, contents))
        // stream.write_all(resp.as_bytes()).unwrap()
    }
}
