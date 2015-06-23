use std::collections::HashMap;
use std::str::from_utf8;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HttpMethod {
	Get,
	Post,
	Head,
	Put,
	Delete,
	Options
}

#[derive(Debug, Eq, PartialEq)]
pub struct HttpRequestMessage {
	 pub method: HttpMethod,
	 pub http_version: String,
	 pub url: String,
	 pub headers: HashMap<String, String>,
	 pub body: Vec<u8>
}

impl HttpRequestMessage {
	pub fn empty() -> HttpRequestMessage {
		HttpRequestMessage {
			method: HttpMethod::Get,
			http_version: "".to_string(),
			url: "".to_string(),
			headers: HashMap::new(),
			body: Vec::new()
		}
	}
}

#[derive(Debug)]
pub struct HttpResponseMessage {
	pub response_code: u16,
	pub response_status: String,
	pub http_version: String,
	pub headers: HashMap<String, String>,
	pub body: Vec<u8>
}

impl HttpResponseMessage {
	pub fn to_bytes(&self) -> Vec<u8> {
		let mut ret = Vec::new();
		
		fn output_line(r: &mut Vec<u8>, s: &str) {
			let nl = b"\r\n";
			for b in s.bytes() {
				r.push(b);
			}
			for b in nl {
				r.push(*b);
			}
		}

		output_line(&mut ret, format!("HTTP {} {} {}", self.http_version, self.response_code, self.response_status).as_str());		

		for (key, val) in &self.headers {
			output_line(&mut ret, format!("{}: {}", key, val).as_str());
		}

		output_line(&mut ret, "");

		ret.push_all(&self.body);

		ret
	}

	pub fn html_utf8(body: &str) -> HttpResponseMessage {
		let mut headers = HashMap::new();
		headers.insert("Content-Type".to_string(), "text/html; charset=UTF-U8".to_string());

		HttpResponseMessage {
			response_code: 200,
			response_status: "OK".to_string(),
			http_version: "1.1".to_string(),
			headers: headers,
			body: body.bytes().collect()
		}
	}
}

