use std::fs::{self, File};
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle, Thread};
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod http;
use http::{HttpMethod, HttpRequest, HttpResp, HttpStatus, HttpVersion};

enum ThreadStatus {
    Idle,
    Busy,
}

type Task = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
struct Worker {
    thread: JoinHandle<()>,
    id: usize,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Task>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let task = receiver.lock().unwrap().recv().unwrap();
            task();
        });

        Self { id, thread }
    }

    pub fn run_task() {}
}
struct ThreadPool {
    // threads: Vec<(Sender<u8>, ThreadStatus)>,
    workers: Vec<Worker>,
    sender: Sender<Task>,
}

impl ThreadPool {
    pub fn new(max_threads: usize) -> Self {
        assert!(max_threads > 0);
        let (sender, recv) = channel();

        let mut workers: Vec<Worker> = Vec::with_capacity(max_threads);

        let recv = Arc::new(Mutex::new(recv));
        for i in 0..max_threads {
            workers.push(Worker::new(i, Arc::clone(&recv)));
        }
        println!("workers: {:#?}", workers);
        Self { workers, sender }
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}
fn main() {
    let ip = "127.0.0.1:7878";
    let listener = TcpListener::bind(ip).unwrap(); // listen to home IP at port 7878

    let mut pool = ThreadPool::new(4);
    println!("Opened connection at: http://{}", ip);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
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
            "/sleep" => {
                std::thread::sleep(std::time::Duration::from_secs(5));
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
