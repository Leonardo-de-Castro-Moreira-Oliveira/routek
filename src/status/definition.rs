use super::{HttpStatusBuilder, HttpStatusManager};

#[derive(Clone)]
pub struct HttpStatus {
    version: String,
    value: u16,
    sign: String,
}

impl HttpStatusBuilder for HttpStatus {
    fn new(version: String, value: u16) -> Option<Self> {
        if let Some(sign) = HttpStatus::get_sign(value) {
            Some(HttpStatus {
                version,
                value,
                sign,
            })
        } else {
            None
        }
    }

    fn build(version: &str, value: u16) -> Option<Self> {
        if let Some(sign) = HttpStatus::get_sign(value) {
            Some(HttpStatus {
                version: version.to_string(),
                value,
                sign,
            })
        } else {
            None
        }
    }

    fn empty(version: &str) -> Self {
        HttpStatus {
            version: version.to_string(),
            value: 404,
            sign: "Not Found".to_string(),
        }
    }
}

impl HttpStatusManager for HttpStatus {
    fn set_version(&mut self, version: &str) -> &mut Self {
        self.version = version.to_string();
        self
    }

    fn set_value(&mut self, value: u16) -> Option<&mut Self> {
        if let Some(sign) = HttpStatus::get_sign(value) {
            self.value = value;
            self.sign = sign;
            Some(self)
        } else {
            None
        }
    }

    fn get_sign(value: u16) -> Option<String> {
        match value {
            100 => Some("Continue".to_string()),
            101 => Some("Switching Protocols".to_string()),
            102 => Some("Processing".to_string()),
            200 => Some("OK".to_string()),
            201 => Some("Created".to_string()),
            202 => Some("Accepted".to_string()),
            203 => Some("Non-Authoritative Information".to_string()),
            204 => Some("No Content".to_string()),
            205 => Some("Reset Content".to_string()),
            206 => Some("Partial Content".to_string()),
            207 => Some("Multi-Status".to_string()),
            208 => Some("Already Reported".to_string()),
            226 => Some("IM Used".to_string()),
            300 => Some("Multiple Choices".to_string()),
            301 => Some("Moved Permanently".to_string()),
            302 => Some("Found".to_string()),
            303 => Some("See Other".to_string()),
            304 => Some("Not Modified".to_string()),
            305 => Some("Use Proxy".to_string()),
            307 => Some("Temporary Redirect".to_string()),
            308 => Some("Permanent Redirect".to_string()),
            400 => Some("Bad Request".to_string()),
            401 => Some("Unauthorized".to_string()),
            402 => Some("Payment Required".to_string()),
            403 => Some("Forbidden".to_string()),
            404 => Some("Not Found".to_string()),
            405 => Some("Method Not Allowed".to_string()),
            406 => Some("Not Acceptable".to_string()),
            407 => Some("Proxy Authentication Required".to_string()),
            408 => Some("Request Timeout".to_string()),
            409 => Some("Conflict".to_string()),
            410 => Some("Gone".to_string()),
            411 => Some("Length Required".to_string()),
            412 => Some("Precondition Failed".to_string()),
            413 => Some("Payload Too Large".to_string()),
            414 => Some("URI Too Long".to_string()),
            415 => Some("Unsupported Media Type".to_string()),
            416 => Some("Range Not Satisfiable".to_string()),
            417 => Some("Expectation Failed".to_string()),
            418 => Some("I'm a teapot".to_string()),
            421 => Some("Misdirected Request".to_string()),
            422 => Some("Unprocessable Entity".to_string()),
            423 => Some("Locked".to_string()),
            424 => Some("Failed Dependency".to_string()),
            425 => Some("Too Early".to_string()),
            426 => Some("Upgrade Required".to_string()),
            428 => Some("Precondition Required".to_string()),
            429 => Some("Too Many Requests".to_string()),
            431 => Some("Request Header Fields Too Large".to_string()),
            451 => Some("Unavailable For Legal Reasons".to_string()),
            500 => Some("Internal Server Error".to_string()),
            501 => Some("Not Implemented".to_string()),
            502 => Some("Bad Gateway".to_string()),
            503 => Some("Service Unavailable".to_string()),
            504 => Some("Gateway Timeout".to_string()),
            505 => Some("HTTP Version Not Supported".to_string()),
            506 => Some("Variant Also Negotiates".to_string()),
            507 => Some("Insufficient Storage".to_string()),
            508 => Some("Loop Detected".to_string()),
            510 => Some("Not Extended".to_string()),
            511 => Some("Network Authentication Required".to_string()),
            _ => None,
        }
    }
}

impl ToString for HttpStatus {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.version, self.value, self.sign)
    }
}
