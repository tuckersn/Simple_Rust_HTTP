use async_std::net::TcpStream;
use futures::io::{AsyncBufReadExt, BufReader};
use std::{result::Result};
use std::io::{Error, ErrorKind};

pub async fn read_till_next(buf: &mut BufReader<&TcpStream>, delimiter: char) -> Result<Vec<u8>, Error> {
	let mut out: Vec<u8> = Vec::new();

	
	match buf.read_until(delimiter as u8, &mut out).await {
		Ok(_) => {

			// println!("RAW INCOMING: {:?}", out );

			if out.len() > 0 {
				let mut out = &out[ 0..out.len()-delimiter.len_utf8() ];
				// CRLF handling, this may be an issue for future
				if out[out.len()-1] == b'\r' {
					out = &out[0..out.len()-1];
				}
				Ok(out.to_vec())
			} else {
				Ok(out.to_vec())
			}
		}
		Err(e) => {
			Err(e)
		}
	}
}

pub async fn read_till_next_string(buf: &mut BufReader<&TcpStream>, delimiter: char) -> Result<String, Error> {
	Ok(String::from_utf8_lossy(read_till_next(buf, delimiter).await?.as_slice()).to_string().to_owned())
}

pub async fn read_next_line(buf: &mut BufReader<&TcpStream>) -> Result<Vec<u8>, Error> {
	read_till_next(buf,'\n').await
}