use core::str::from_utf8;
use http::*;
use core::prelude::*;
use collections::vec::*;
use collections::String;
use collections::string::ToString;

pub struct HttpRequestParser {
    buffer: Vec<u8>,
    pos: usize,
    line_num: u16,
    headers_parsed: bool,
    msg: HttpRequestMessage,
}

#[derive(Debug)]
pub enum HttpRequestParserState {
    MoreDataRequired,
    Complete,
}

#[derive(Debug)]
pub enum HttpRequestParserError {
    InvalidString,
    HeaderError,
    LineParseError(String),
}

impl HttpRequestParser {
    pub fn new() -> HttpRequestParser {
        HttpRequestParser {
            buffer: Vec::new(),
            pos: 0,
            line_num: 0,
            headers_parsed: false,
            msg: HttpRequestMessage::empty()
        }
    }

    pub fn is_first_line_parsed(&self) -> bool {
        self.line_num > 0
    }

    pub fn are_headers_parsed(&self) -> bool {
        self.headers_parsed
    }

    pub fn read_how_many_bytes(&self) -> u32 {
        if self.is_first_line_parsed() && self.are_headers_parsed() {
            if self.get_request().method == HttpMethod::Get || self.get_request().method == HttpMethod::Head {
                return 0;
            }

            let cl = self.msg.content_length();
            if cl.is_some() {
                return cl.unwrap() - self.msg.body.len() as u32;
            }
        }

        return 1;
    }

    pub fn parse_bytes(&mut self, data: &[u8]) -> Result<HttpRequestParserState, HttpRequestParserError> {
        if data.len() == 0 { return Ok(HttpRequestParserState::MoreDataRequired); }

        self.buffer.push_all(data);

        if self.headers_parsed == false {
            let p = self.pos;
            for i in p..self.buffer.len(){
                //println!("i = {}", i);
                let f = self.buffer[i];
                if f == '\r' as u8 {
                    if i + 1 < self.buffer.len() {
                        let f2 = self.buffer[i + 1];
                        if f2 == '\n' as u8 {
                            // line found
                            let line = &self.buffer[self.pos..i];
                            //println!("line: [{}..{}]", self.pos, i);

                            self.pos = i + 2;							

                            if line.len() == 0 {
                                self.line_num += 1;
                                self.headers_parsed = true;
                                break;
                            }

                            if self.line_num == 0 {
                                try!(HttpRequestParser::parse_first_line(&mut self.msg, line));
                            } else {
                                try!(HttpRequestParser::parse_line(&mut self.msg, line));
                            }

                            self.line_num += 1;
                        }
                    }
                }
            }
        }

        if self.headers_parsed {
            {
                let s = &self.buffer[(self.pos)..];
                self.msg.body.push_all(s);
            }
            self.buffer.clear();
            self.pos = 0;
        }


        Ok(HttpRequestParserState::MoreDataRequired)
    }

    fn parse_first_line(msg: &mut HttpRequestMessage, line: &[u8]) -> Result<(), HttpRequestParserError> {
        let str = from_utf8(line);
        if !str.is_ok() { return Err(HttpRequestParserError::InvalidString); }
        let str = str.unwrap();

        let mut middle = str;

        if str.starts_with("GET") {
            msg.method = HttpMethod::Get;
            middle = &str[4..];
        } else if str.starts_with("HEAD") {
            msg.method = HttpMethod::Head;
            middle = &str[5..];
        } else if str.starts_with("POST") {
            msg.method = HttpMethod::Post;
            middle = &str[5..];
        } else {
            return Err(HttpRequestParserError::LineParseError(str.to_string()));
        }

        if str.ends_with("HTTP/1.1") {
            msg.http_version = "1.1".to_string();
        } else if str.ends_with("HTTP/1.0") {
            msg.http_version = "1.0".to_string();
        }

        let l = middle.rfind("HTTP/1");
        if l.is_none() { return Err(HttpRequestParserError::LineParseError(str.to_string())); }

        let url = &middle[..(l.unwrap() - 1)];
        msg.url = url.to_string();

        return Ok(());
    }

    fn parse_line(msg: &mut HttpRequestMessage, line: &[u8]) -> Result<(), HttpRequestParserError> {
        let str = from_utf8(line);
        if str.is_ok() {
            let str = str.unwrap();

            let sep = str.find(": ");
            if sep.is_none() {
                return Err(HttpRequestParserError::HeaderError);
            }
            let sep = sep.unwrap();

            let key = &str[0..sep];
            let val = &str[sep + 2..];

            msg.headers.insert(key.to_string(), val.to_string());

        } else {
            msg.http_version = "fail".to_string();
            return Err(HttpRequestParserError::InvalidString);
        }

        return Ok(());
    }

    pub fn get_request(&self) -> &HttpRequestMessage {
        &self.msg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::prelude::*;
    use std::prelude::*;
    use collections::vec::Vec;

    #[test]
    pub fn test_request_parsing() {
        let msg = "GET /index.html HTTP/1.1\r\nHost: www.example.com\r\n\r\nbody";

        let mut parser = HttpRequestParser::new();
        let bytes = &msg.bytes();
        let bytes: Vec<u8> = bytes.clone().collect();
        parser.parse_bytes(&bytes).unwrap();
        /*
           for b in msg.bytes() {
           parser.parse_bytes(&[b]);
           }
           */

        let req = parser.get_request();
        println!("parsed: {:?}", req);
    }
}
