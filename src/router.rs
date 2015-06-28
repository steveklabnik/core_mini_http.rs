
use http::*;
use core::prelude::*;
use collections::vec::*;
use collections::String;
use collections::string::ToString;
use alloc::boxed::Box;

#[derive(Debug, Eq, PartialEq)]
pub enum HttpRouteError {
	NotApplicable,
	ProcessingError,
	NoRouteFound
}

pub trait HttpRoute {
	fn try(&self, msg: &HttpRequestMessage) -> Result<bool, HttpRouteError>;
	fn execute(&self, msg: &HttpRequestMessage) -> Result<HttpResponseMessage, HttpRouteError>;
}

pub struct HttpRouteStaticUrl {
	pub urls: Vec<String>,
	pub methods: Vec<HttpMethod>,

	pub action: Box<Fn(&HttpRequestMessage) -> HttpResponseMessage + Send + Sync>
}

impl HttpRouteStaticUrl {
	pub fn new_get<F>(url: &str, action: F) -> HttpRouteStaticUrl 
		where F: Fn(&HttpRequestMessage) -> HttpResponseMessage + Send + Sync + 'static
	{
		HttpRouteStaticUrl {
			urls: vec![url.to_string()],
			methods: vec![HttpMethod::Get],
			action: Box::new(action)
		}
	}
}

impl HttpRoute for HttpRouteStaticUrl {
	fn try(&self, msg: &HttpRequestMessage) -> Result<bool, HttpRouteError> {
		if self.urls.contains(&msg.url) && self.methods.contains(&msg.method) {
			return Ok(true);
		}

		return Ok(false);
	}

	fn execute(&self, msg: &HttpRequestMessage) -> Result<HttpResponseMessage, HttpRouteError> {
		//Err(HttpRouteError::ProcessingError)
		Ok(self.action.call((msg,)))
	}
}



pub fn http_router<'a>(routes: &'a [Box<HttpRoute + Send + Sync + 'static>], req: &HttpRequestMessage) -> Result<&'a Box<HttpRoute + Send + Sync + 'static>, HttpRouteError> {
	for route in routes {
		let t = route.try(&req);
		if t.is_ok() && t.unwrap() == true {
			return Ok(route);
		}
	}

	return Err(HttpRouteError::NoRouteFound);
}
