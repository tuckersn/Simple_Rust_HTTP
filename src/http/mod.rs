pub mod request;
pub mod server;
pub mod response;
pub mod routes;
pub mod stream;

use std::{fmt, io::{Error, ErrorKind, Result}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum HttpMethod {
	GET,
	POST,
	OPTIONS
}

impl HttpMethod {
	fn from_str(s: impl Into<String>) -> Result<HttpMethod> {
		let s: String = s.into();
		HttpMethod::from_bytes(s.as_bytes())
	}

	fn from_bytes(s: impl Into<Vec<u8>>) -> Result<HttpMethod> {
		let s: Vec<u8> = s.into();
		match s.as_slice() {
			b"GET" => Ok(HttpMethod::GET),
			b"GET " => Ok(HttpMethod::GET),
			b"POST" => Ok(HttpMethod::POST),
			b"POST " => Ok(HttpMethod::POST),
			b"OPTIONS" => Ok(HttpMethod::OPTIONS),
			b"OPTIONS " => Ok(HttpMethod::OPTIONS),
			_ => Err(Error::new(ErrorKind::NotFound, format!("Invalid method type specified: {:?}", s)))
		}
	} 
}

impl fmt::Debug for HttpMethod {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", serde_json::to_string(&self).unwrap())
	}
}

impl fmt::Display for HttpMethod {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", serde_json::to_string(&self).unwrap())
	}
}
