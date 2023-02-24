use std::{
    fs,
    io::{prelude::*, BufRead, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use ex_20_webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("localhost:3001").unwrap();
    let pool = ThreadPool::new(10);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "public/index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "public/index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "public/404.html"),
    };

    let body = fs::read_to_string(file_name).unwrap();
    let length = body.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{body}");

    stream.write_all(response.as_bytes()).unwrap();
}
