use std::{collections::HashMap, pin::Pin, fmt};
use async_std::prelude::*;
use async_std::net::TcpStream;
use async_std::stream::{StreamExt};
use serde::Serialize;
use std::result::Result;
use std::io;

pub struct Response {
	pub status: Option<u16>,
	pub status_message: Option<String>,
	pub headers: HashMap<String, String>,
	pub body: Option<Vec<u8>>,
	stream: Box<TcpStream>
}



impl Response {

	pub fn new(stream: &TcpStream) -> Response {
		Response {
			status: None,
			status_message: None,
			headers: HashMap::new(),
			body: None,
			stream: Box::new(stream.to_owned())
		}
	}

	pub fn body(&mut self, body: impl Into<Vec<u8>>) -> &mut Response {
		self.body = Some(body.into());
		self
	}

	pub fn header(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Response {
		self.headers.insert(key.into(), value.into());
		self
	}

	pub fn status(&mut self, code: u16) -> &mut Response {
		self.status = Some(code);
		self
	}

	pub fn status_with_msg(&mut self, code: u16, text: Option<&str>) -> &mut Response {
		self.status = Some(code);
		match text {
			Some(text) => {
				self.status_message = Some(text.to_string());
			}
			None => {
				self.status_message = None;
			}
		}
		self
	}


	pub async fn send(&mut self) -> Result<(), io::Error> {

		let mut stream = self.stream.as_ref();

		stream.write_all(format!("HTTP/1.1 {} {}\n", self.status.unwrap_or(500), self.status_message.to_owned().unwrap_or("".to_string()) ).as_bytes() ).await?;

		for (name, value) in &self.headers {
			stream.write_all( format!("{}: {}\n", name, value).as_bytes() ).await?;
		}

		match &self.body {
			Some(body) => {
				stream.write_all(format!("Content-Length: {}\n", body.len()).as_bytes()).await?;
				stream.write_all(b"\n").await?;
				stream.write_all(body.as_ref()).await?;
			}
			None => {
				stream.write_all(b"\n").await?;
			}
		}
		
		Ok(())
	}

	pub async fn send_html(&mut self, content: impl Into<String>) -> Result<(), io::Error> {
		let content: String = content.into();
		self.header("Content-Type", "text/html");
		self.body(content.as_bytes().to_vec());
		self.send().await
	}


	pub async fn send_json<Input>(&mut self, content: &Input) -> Result<(), io::Error>
	where Input: Serialize
	{
		self.header("Content-Type", "application/json");
		self.body(serde_json::to_string(&content).unwrap().as_bytes());
		self.send().await
	}
}