use futures::io::{BufReader};
use async_std::net::{TcpStream};
use std::io::{Error, ErrorKind};
use std::{collections::HashMap, fmt};
use std::str;
use serde::{Serialize, Deserialize};
use std::result::Result;
use futures::io::{AsyncBufReadExt};


use crate::{http::HttpMethod};
use super::stream::{read_next_line, read_till_next, read_till_next_string};


#[derive(Serialize, Deserialize)]
pub struct Request {
	pub method: HttpMethod,
	pub path: String,
	pub headers: HashMap<String, String>,
	pub query: HashMap<String, String>,
	pub params: HashMap<String, String>,
	pub body: Vec<u8>
}


impl fmt::Display for Request {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", serde_json::to_string(&self).unwrap())
	}
}






impl Request {

	pub async fn from_stream(stream: &TcpStream) -> Result<Request, Error> {

		let mut buf_reader = BufReader::new(stream);

		let first_line = read_next_line(&mut buf_reader).await?;
		if first_line.len() < b"GET / HTTP/1.1".len() {
			return Err(Error::new(ErrorKind::InvalidInput, format!("Very little data sent over TCP, length of request was {}", first_line.len())))
		}
		let mut first_line = first_line.split(|e| *e == b' ' || *e == b'\n' );
	
		let method = HttpMethod::from_bytes(first_line.next().unwrap())?;
		let path = first_line.next().unwrap();
		let version = first_line.next().unwrap();

		match version {

			b"HTTP/1.1" => {

				let mut headers: HashMap<String, String> = HashMap::new();
				loop {
					match read_till_next(&mut buf_reader, '\n').await {
						Ok(line) => {
							if line.len() < 2 {
								break;
							} else {
								let pos = line.as_slice().iter().position(|byte| *byte == b':').unwrap();
								headers.insert(
									String::from_utf8_lossy(&line.as_slice()[0..pos]).into_owned(),
									String::from_utf8_lossy(&line.as_slice()[pos + 2..]).into_owned()
								);
							}
						}
						Err(e) => {
							return Err(e)
						}
					}
				}
 
				Ok(Request {
					path: String::from_utf8(path.to_vec()).unwrap(),
					method: method,
					headers: headers,
					query: HashMap::new(),
					params: HashMap::new(),
					body: buf_reader.buffer().to_vec()
				})

			},

			_ => {
				Err(Error::new(ErrorKind::InvalidData, "Invalid HTTP version parsed, expected HTTP/1.1"))
			}

		}
	}

}

