use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::net::Shutdown;

extern crate core_mini_http;

use core_mini_http::*;



fn main() {
    let listener = TcpListener::bind("127.0.0.1:8088").unwrap();

    fn handle_client(stream: TcpStream) {
        println!("handling client");
        let mut stream = stream;

        let mut parser = HttpRequestParser::new();

        loop {
            let mut buf = [0; 1];
            let r = stream.read(&mut buf);
            if !r.is_ok() {
                panic!("stream broken");
            }

            let parsed = parser.parse_bytes(&buf);
            if !parsed.is_ok() {
                panic!("parser borked");
            }

            if parser.is_first_line_parsed() && parser.are_headers_parsed() && parser.get_request().method == HttpMethod::Get {
                //println!("get finished!");
                break;
            }
        }


        //let mut buf = Vec::new();
        //stream.read_to_end(&mut buf);        
        println!("stream read");

        println!("{:?}", parser.get_request());

        /*
        let res = parser.parse_bytes(&buf);
        if res.is_err() {
            println!("err: {:?}", res);
        }

        println!("{:?}", parser.get_request());
        */
        stream.shutdown(Shutdown::Read);



        stream.write(b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<h1>Hello World!</h1>\r\n");
        stream.flush();
        stream.shutdown(Shutdown::Write);

    }


    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream);                    
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }

    // close the socket server
    drop(listener);
}