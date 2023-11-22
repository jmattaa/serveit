pub enum HTTPMethod {
    GET,
    POST,
    // TODO: ADD MORE
}

impl HTTPMethod {
    pub fn from_str(string: &str) -> Option<Self> {
        match string.to_uppercase().as_str() {
            "GET" => Some(Self::GET),
            "POST" => Some(Self::POST),
            _ => None,
        }
    }
}

pub struct HTTPRequest {
    method: HTTPMethod,
    path: String,
    version: String,
    // headers: HTTPHeader,
    // body: String,
}

impl HTTPRequest {
    // getters 
    pub fn method(&self) -> &HTTPMethod { &self.method }
    pub fn path(&self) -> &String { &self.path }
    pub fn version(&self) -> &String { &self.version }

    pub fn parse(req_raw: &str) -> Self {
        let lines: Vec<&str> = req_raw.lines().collect();

        if let Some(request_line) = lines.first() {
            let parts: Vec<&str> = request_line.split_whitespace().collect();

            if parts.len() == 3 {
                let method = HTTPMethod::from_str(parts[0]).unwrap();
                let path = parts[1].to_owned();
                let version = parts[2].to_owned();

                return Self {
                    method,
                    path,
                    version,
                };
            }
        }
        panic!("Request not valid");
    }
}

