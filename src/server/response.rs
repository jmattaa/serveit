use crate::server::utils::{CRLF, DBL_CRLF};

pub struct HTTPResponse {
    version: String,
    status: String,
    content_type: String,
    content: String,
    // headers,
    // body
} 

impl HTTPResponse {
    pub fn new(version: String, status: String, 
               content_type: String, content: String) -> Self {
        Self {
            version,
            status,
            content_type,
            content,
        } 
    }

    pub fn construct(&self) -> String {
        format!(
            "{} {} {CRLF}Content-Type: {}{CRLF}Content-Length: {}{DBL_CRLF}{}",
            self.version,
            self.status,
            self.content_type,
            self.content.len(),
            self.content,
        )
    }
}
