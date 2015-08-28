use http::*;
use collections::vec::*;
use collections::String;
use collections::string::ToString;
use collections::BTreeMap;
use alloc::boxed::Box;

#[derive(Debug, Eq, PartialEq)]
pub enum HttpRouteError {
    NotApplicable,
    ProcessingError,
    NoRouteFound,
}

pub trait HttpRoute {
    fn try(&self, msg: &HttpRequestMessage) -> Result<bool, HttpRouteError>;
    fn execute(&self, msg: &HttpRequestMessage) -> Result<HttpResponseMessage, HttpRouteError>;
}

#[derive(Debug, Eq, PartialEq)]
pub enum DynamicUrlPart {
    Static(String),
    UrlVar(String),
}

#[derive(Debug)]
pub struct DynamicUrl {
    parts: Vec<DynamicUrlPart>,
}

impl DynamicUrl {
    pub fn new(parts: Vec<DynamicUrlPart>) -> DynamicUrl {

        // remove the trailing slash, not really neaded...
        let mut parts = parts;
        if parts.len() > 1 && parts.ends_with(&[DynamicUrlPart::Static("/".to_string())]) {
            parts.pop();
        }

        DynamicUrl {
            parts: parts
        }
    }

    /// rails-like syntax
    /// /test/:id
    pub fn parse_str(s: &str) -> Option<DynamicUrl> {
        let mut parts = Vec::new();

        let mut p = s;
        loop {
            let f = p.find(":");
            if f.is_some() {
                let idx = f.unwrap();
                let st = DynamicUrlPart::Static(p[..idx].to_string());
                parts.push(st);

                let other = &p[(idx+1)..];
                let sep = other.find("/");
                if sep.is_some() {
                    let sep_idx = sep.unwrap();
                    let v = DynamicUrlPart::UrlVar(other[..sep_idx].to_string());
                    parts.push(v);

                    p = &other[sep_idx..];
                } else {
                    let v = DynamicUrlPart::UrlVar(other.to_string());
                    parts.push(v);
                    break;
                }
            } else {
                let st = DynamicUrlPart::Static(p.to_string());
                parts.push(st);
                break;
            }
        }

        if parts.len() > 0 {
            Some(DynamicUrl::new(parts))
        } else {
            None
        }
    }

    pub fn match_url(&self, url: &str) -> Option<DynamicUrlMatch> {
        let mut is_match = true;
        let mut url_match = DynamicUrlMatch {
            vars: BTreeMap::new()
        };

        let mut u = url;
        let mut i = 0;
        loop {
            let part = &self.parts[i];

            match *part {
                DynamicUrlPart::Static(ref m_url) => {
                    if u.starts_with(m_url) {
                        u = &u[m_url.len()..];
                    } else {
                        is_match = false;
                        break;
                    }
                },
                DynamicUrlPart::UrlVar(ref var_name) => {
                    if u.len() == 0 {
                        is_match = false;
                        break;
                    } else {
                        let m = u.splitn(1, "/").collect::<Vec<_>>();
                        if m.len() == 2 {
                            url_match.vars.insert(var_name.clone(), m[0].to_string());
                            u = m[1];
                        } else if u.ends_with("/") {
                            url_match.vars.insert(var_name.clone(), u[..(u.len()-1)].to_string());
                            u = &u[..(u.len()-1)];
                        } else if i == self.parts.len() - 1 {
                            // we're the last one, grab everything if possible
                            let sep = u.find("/");
                            if sep.is_some() {
                                is_match = false;
                                break;
                            } else if u.len() > 0 {
                                url_match.vars.insert(var_name.clone(), u.to_string());
                            } else {
                                is_match = false;
                                break;
                            }
                        } else if u.len() == 0 {
                            is_match = false;
                            break;
                        }
                    }
                }
            }

            i += 1;
            if i == self.parts.len() { break; }
        }

        // todo: check if all required variables are non-empty!
        // todo: url decode vars

        if is_match {
            Some(url_match)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct DynamicUrlMatch {
    vars: BTreeMap<String, String>,
}

impl DynamicUrlMatch {
    pub fn get(&self, key: &str) -> Option<&String> {
        self.vars.get(key)
    }
}

pub struct HttpRouteDynamicUrl {
    pub url: DynamicUrl,
    pub methods: Vec<HttpMethod>,
    pub action: Box<Fn(&HttpRequestMessage, &DynamicUrlMatch) -> HttpResponseMessage + Send + Sync>,
}

impl HttpRouteDynamicUrl {
    pub fn new<F>(url: DynamicUrl, method: HttpMethod, action: F) -> HttpRouteDynamicUrl
        where F: Fn(&HttpRequestMessage, &DynamicUrlMatch) -> HttpResponseMessage + Send + Sync + 'static
        {
            HttpRouteDynamicUrl {
                url: url,
                methods: vec![method],
                action: Box::new(action)
            }
        }
}

impl HttpRoute for HttpRouteDynamicUrl {
    fn try(&self, msg: &HttpRequestMessage) -> Result<bool, HttpRouteError> {
        let m = self.url.match_url(&msg.url);
        if m.is_some() && self.methods.contains(&msg.method) {
            return Ok(true);
        }

        return Ok(false);
    }

    fn execute(&self, msg: &HttpRequestMessage) -> Result<HttpResponseMessage, HttpRouteError> {
        let m = self.url.match_url(&msg.url);

        if m.is_some() {
            Ok((self.action)(msg, &m.unwrap()))
        } else {
            Err(HttpRouteError::NoRouteFound)
        }
    }
}

pub struct HttpRouteStaticUrl {
    pub urls: Vec<String>,
    pub methods: Vec<HttpMethod>,
    pub action: Box<Fn(&HttpRequestMessage) -> HttpResponseMessage + Send + Sync>,
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
        Ok((self.action)(msg))
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

#[cfg(test)]
mod tests {
    use super::*;

    use collections::string::ToString;

    #[test]
    pub fn test_dynamic_urls() {
        {
            let route_parse = DynamicUrl::parse_str("/test/:id/");
            println!("route: {:?}", route_parse);
            {
                let r = route_parse.unwrap();

                let m = r.match_url("/test/123/");
                println!("match trailing: {:?}", m);

                let m = r.match_url("/test/123/xyz");
                println!("should fail with trailing content: {:?}", m);
            }
            let route_parse = DynamicUrl::parse_str("/test/:id");
            println!("route: {:?}", route_parse);

        }

        {
            let p = vec![DynamicUrlPart::Static("/test/".to_string()), DynamicUrlPart::UrlVar("id".to_string())];
            let d = DynamicUrl::new(p);
            println!("dynamic url: {:?}", d);

            let m = d.match_url("/test/123/");
            println!("url match: {:?}", m);
            let m = d.match_url("/test/123");
            println!("url match: {:?}", m);
            let m = d.match_url("/test/");
            println!("url match: {:?}", m);
            let m = d.match_url("/");
            println!("url match: {:?}", m);
        }
    }
}
