use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum HttpVersion {
    V(f32),
}

impl HttpVersion {
    pub fn as_str(&self) -> &str {
        match self {
            HttpVersion::V(1.0) => "HTTP/1.0",
            HttpVersion::V(1.1) => "HTTP/1.1",
            HttpVersion::V(2.0) => "HTTP/2.0",
            HttpVersion::V(3.0) => "HTTP/3.0",
            _ => unreachable!(
                "Must be a valid HTTP version, if new version added, update enum to handle"
            ),
        }
    }

    pub fn from_string(string: &str) -> Result<Self, String> {
        match string {
            "HTTP/1.0" => Ok(HttpVersion::V(1.0)),
            "HTTP/1.1" => Ok(HttpVersion::V(1.1)),
            "HTTP/2.0" => Ok(HttpVersion::V(2.0)),
            "HTTP/3.0" => Ok(HttpVersion::V(3.0)),
            _ => Err(String::from("Invalid HTTP version string")),
        }
    }
}

pub enum HttpStatus {
    // 1xx Informational
    Continue,
    SwitchingProtocols,

    // 2xx Success,
    Ok,
    Created,
    Accepted,

    // 3xx Redirection
    MovedPermanently,
    Found,
    NotModified,

    // 4xx Client
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Teapot,

    // 5xx Server
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
}

impl HttpStatus {
    pub fn from_u16(code: u16) -> Result<Self, String> {
        match code {
            100 => Ok(HttpStatus::Continue),
            101 => Ok(HttpStatus::SwitchingProtocols),
            200 => Ok(HttpStatus::Ok),
            201 => Ok(HttpStatus::Created),
            202 => Ok(HttpStatus::Accepted),
            301 => Ok(HttpStatus::MovedPermanently),
            302 => Ok(HttpStatus::Found),
            304 => Ok(HttpStatus::NotModified),
            400 => Ok(HttpStatus::BadRequest),
            401 => Ok(HttpStatus::Unauthorized),
            403 => Ok(HttpStatus::Forbidden),
            404 => Ok(HttpStatus::NotFound),
            418 => Ok(HttpStatus::Teapot),
            500 => Ok(HttpStatus::InternalServerError),
            501 => Ok(HttpStatus::NotImplemented),
            502 => Ok(HttpStatus::BadGateway),
            503 => Ok(HttpStatus::ServiceUnavailable),
            _ => Err(String::from("Unimplemented or invalid HTTP response code")),
        }
    }

    pub fn numeric_code(&self) -> u16 {
        match self {
            HttpStatus::Continue => 100,
            HttpStatus::SwitchingProtocols => 101,
            HttpStatus::Ok => 200,
            HttpStatus::Created => 201,
            HttpStatus::Accepted => 202,
            HttpStatus::MovedPermanently => 301,
            HttpStatus::Found => 302,
            HttpStatus::NotModified => 304,
            HttpStatus::BadRequest => 400,
            HttpStatus::Unauthorized => 401,
            HttpStatus::Forbidden => 403,
            HttpStatus::NotFound => 404,
            HttpStatus::Teapot => 418,
            HttpStatus::InternalServerError => 500,
            HttpStatus::NotImplemented => 501,
            HttpStatus::BadGateway => 502,
            HttpStatus::ServiceUnavailable => 503,
        }
    }

    pub fn reason_phrase(&self) -> &'static str {
        match self {
            HttpStatus::Continue => "Continue",
            HttpStatus::SwitchingProtocols => "Switching Protocols",
            HttpStatus::Ok => "OK",
            HttpStatus::Created => "Created",
            HttpStatus::Accepted => "Accepted",
            HttpStatus::MovedPermanently => "Moved Permanently",
            HttpStatus::Found => "Found",
            HttpStatus::NotModified => "Not Modified",
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::Unauthorized => "Unauthorized",
            HttpStatus::Forbidden => "Forbidden",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::Teapot => "I'm a teapot",
            HttpStatus::InternalServerError => "Internal Server Error",
            HttpStatus::NotImplemented => "Not Implemented",
            HttpStatus::BadGateway => "Bad Gateway",
            HttpStatus::ServiceUnavailable => "Service Unavailable",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::CONNECT => "CONNECT",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::TRACE => "TRACE",
            HttpMethod::PATCH => "PATCH",
        }
    }

    pub fn from_string(string: &str) -> Result<Self, String> {
        match string {
            "GET" => Ok(HttpMethod::GET),
            "HEAD" => Ok(HttpMethod::HEAD),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "CONNECT" => Ok(HttpMethod::CONNECT),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            "TRACE" => Ok(HttpMethod::TRACE),
            "PATCH" => Ok(HttpMethod::PATCH),
            _ => Err(String::from("Invalid HTTP method string")),
        }
    }
}

type HttpHeaders = HashMap<String, String>;
pub struct HttpResp {
    version: HttpVersion,
    status_code: HttpStatus,
    headers: Option<HttpHeaders>,
    body: Option<String>,
}

impl HttpResp {
    pub fn new(
        version: HttpVersion,
        status_code: HttpStatus,
        headers: Option<HttpHeaders>,
        body: Option<String>,
    ) -> Self {
        Self {
            version,
            status_code,
            headers,
            body,
        }
    }

    pub fn from_string(response: &str) -> Result<Self, String> {
        let mut lines = response.lines();

        // Parse the status line
        let status_line = lines.next().ok_or("Empty response")?;
        let parts: Vec<&str> = status_line.split_whitespace().collect();

        if parts.len() < 3 {
            return Err("Invalid status line".to_string());
        }

        let version = HttpVersion::from_string(parts[0]).expect(&format!(
            "Expect valid HTTP version string, recieved {}",
            parts[0]
        ));
        let status_code: HttpStatus =
            HttpStatus::from_u16(parts[1].parse().expect("Couldn't parse status code"))
                .expect("Expect valid HTML numeric status code");
        let status_text = parts[2..].join(" "); // Join remaining parts for status text

        // Parse headers
        let mut headers = HashMap::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                // End of headers
                break;
            }
            let header_parts: Vec<&str> = line.splitn(2, ": ").collect();
            if header_parts.len() == 2 {
                headers.insert(header_parts[0].to_string(), header_parts[1].to_string());
            }
        }

        // Parse the body (if it exists)
        let body = lines.collect::<Vec<&str>>().join("\n");

        // Construct and return the HttpResp
        Ok(HttpResp {
            version,
            status_code,
            headers: Some(headers),
            body: if body.is_empty() { None } else { Some(body) },
        })
    }

    pub fn with_code(status_code: u16, version: HttpVersion) -> Self {
        let code = HttpStatus::from_u16(status_code)
            .expect("TODO: pass back error. Expect a valid HTTP status code");
        Self::new(version, code, None, None)
    }

    pub fn with_text_html(version: HttpVersion, status_code: u16, body: String) -> Self {
        let code = HttpStatus::from_u16(status_code)
            .expect("TODO: pass back error. Expect a valid HTTP status code");
        let mut headers: HttpHeaders = HashMap::new();
        headers.insert(
            String::from("Content-Type"),
            String::from("text/html; charset=UTF-8"),
        );
        headers.insert(String::from("Content-Length"), format!("{}", body.len()));

        Self::new(version, code, Some(headers), Some(body))
    }

    pub fn to_string(&self) -> String {
        let mut request = format!(
            "{} {} {}\r\n",
            self.version.as_str(),
            self.status_code.numeric_code(),
            self.status_code.reason_phrase(),
        );

        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                request.push_str(&format!("{}: {}\r\n", key, value));
            }
        }
        request.push_str("\r\n"); // newline after headers block

        if let Some(body) = &self.body {
            request.push_str(body);
        }

        request
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    method: HttpMethod,
    uri: String,
    version: HttpVersion,
    headers: Option<HttpHeaders>,
    body: Option<String>,
}

impl HttpRequest {
    pub fn new(
        method: HttpMethod,
        uri: String,
        version: HttpVersion,
        headers: Option<HttpHeaders>,
        body: Option<String>,
    ) -> Self {
        Self {
            method,
            uri,
            version,
            headers,
            body,
        }
    }

    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn from_request(request: &Vec<String>) -> Result<Self, String> {
        let mut lines = request.iter();

        // Parse the request line
        let request_line = lines.next().ok_or("Empty request")?;
        let parts: Vec<&str> = request_line.split_whitespace().collect();

        if parts.len() < 3 {
            return Err("Invalid request line".to_string());
        }

        let method = HttpMethod::from_string(parts[0]).expect("Expected valid HTTP method string");
        let uri = parts[1].to_string();
        let version_string: String = parts[2].parse().expect("Couldn't parse status code");
        let version: HttpVersion =
            HttpVersion::from_string(&version_string).expect("Expect valid HTML version string");
        // Status text is not needed

        // Parse headers
        let mut headers = HashMap::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                // End of headers
                break;
            }
            let header_parts: Vec<&str> = line.splitn(2, ": ").collect();
            if header_parts.len() == 2 {
                headers.insert(header_parts[0].to_string(), header_parts[1].to_string());
            }
        }

        // Parse the body (if it exists)
        // let body = lines.collect::<Vec<&String>>().join("\n");
        let body = lines
            .map(|line| line.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        Ok(HttpRequest {
            method,
            uri,
            version,
            headers: if headers.is_empty() {
                None
            } else {
                Some(headers)
            },
            body: if body.is_empty() { None } else { Some(body) },
        })
    }

    pub fn to_string(&self) -> String {
        let mut request = format!(
            "{} {} {}\r\n",
            self.method.as_str(),
            self.uri,
            self.version.as_str()
        );

        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                request.push_str(&format!("{}: {}\r\n", key, value));
            }
        }

        request.push_str("\r\n"); // newline after headers block

        if let Some(body) = &self.body {
            request.push_str(body);
        }

        request
    }
}
