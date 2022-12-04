use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    env,
};
use gethostname::gethostname;


fn main() {
    let port= match env::var("PING_LISTEN_PORT"){
        Ok(val) => val,
        Err(_) => 8080.to_string(),
    };
    let listener = TcpListener::bind(format!("0.0.0.0:{}",port)).unwrap();
    println!("AAAAAAAAAAAAAA");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    };
}


fn handle_connection(mut stream: TcpStream) {
    println!("OOOOOOOOOOOOOOOOOOOOOOOOOOOO");
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let request_line = &http_request[0];
    if request_line == "GET /ping HTTP/1.1" || request_line == "GET /ping HTTP/1.0"{
        println!("RRRRRRRRRRRRRRRRRRRRRRRRRRRR");
        let contents = header_to_json(http_request);
        let status_line = "HTTP/1.1 200 Ok";
        let length=contents.len();
        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
        stream.write_all(response.as_bytes()).unwrap();
        println!("Hostname: {:?}", gethostname());
    } else {
        let contents = "";
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let length = contents.len();
        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
    
        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn header_to_json(request : Vec<String>) -> String {
    let mut response_body: Vec<String> = Vec::new();
    response_body.push("{".to_string());
    for data in request[1..].iter() {
        let (s1, s2) = data.split_once(": ").unwrap();
        response_body.push(format!(",\"{}\": \"{}\"", s1, s2));
    }
    response_body[1].remove(0);
    response_body.push("}".to_string());
    response_body.join("")
}
