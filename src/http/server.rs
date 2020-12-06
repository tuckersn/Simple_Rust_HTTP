use async_std::{stream::{StreamExt}, task};
use async_std::net::{TcpListener, TcpStream};
use futures::Future;

use std::result::Result;
use std::{io::ErrorKind, io::{Error}, net::Shutdown, sync::Arc};

use crate::http::request::Request;
use crate::http::response::Response;
use crate::http::routes::Router;


async fn error_handling(_req: Request, mut res: Response) -> Result<(), Error> {
	res.status(404).body("404 Error ¯\\_(ツ)_/¯").send().await
}

async fn packet_handler(stream: TcpStream, router: &Router) -> Result<(), Error> {
	
	// Split the stream into a read and write component
	let (mut reader, writer) = &mut (&stream, &stream);

	// Try to build a request from the stream, if this fails it's because
	// the data it's recieving is an invalid HTTP/1.1 packet
	match Request::from_stream(&mut reader).await {

		Ok(request) => {
			let response = Response::new(writer);
			let result: Result<(), Error>;

			match router.find( &request.path ) {
				Ok(node) => {
					match node.function() {
						Some(function) => {
							result = function(request, response).await;
						}
						None => {
							result = error_handling(request, response).await;
						}
					}
				}
				Err(_) => {
					result = error_handling(request, response).await;
				}
			}
			
			match result {
				Ok(_) => {}
				Err(e) => {

					match e.kind() {
						ErrorKind::InvalidInput => { /* potentially empty TCP packet, just ignore */ }
						ErrorKind::InvalidData => {
							println!("Malformed packet was sent: {:?}", e);
						}
						_ => {
							println!("\n-----\nFailed TCP request: {}", e);
						}
					}

					
				}
			}
		
			stream.shutdown(Shutdown::Both).unwrap();
		}

		Err(e) => {
			if e.kind() == ErrorKind::InvalidData {
				println!("Failed to parse a request: {:?}", e);
			}
		}
	}

	
	Ok(())
}

async fn exec(tcp: TcpStream, router: Arc<Router>) {
	packet_handler(tcp, router.as_ref()).await.unwrap();
}



pub struct HttpServer {
	router: Router
}

impl HttpServer {

	pub fn new() -> HttpServer {
		HttpServer {
			router: Router::new()
		}
	}

	pub fn register<RoutingFunction, RoutingFuture>(&mut self, path: &str, function: RoutingFunction) -> &HttpServer
	where
		RoutingFunction: (Fn(Request, Response) -> RoutingFuture) + Send + Sync + 'static,
		RoutingFuture: Future<Output = Result<(), Error>> + Send + Sync + 'static,
	{
		self.router.register(path, function);
		self
	}

	async fn tcp_handler(http_server: HttpServer, listener: TcpListener) -> Result<(), Error> {
		let router = Arc::new(http_server.router);
		let mut incoming = listener.incoming();
		while let Some(stream) = incoming.next().await {
			task::spawn(exec(stream.unwrap(), router.clone()));
		}
		Ok(())
	}

	pub async fn start(self, address: impl Into<String>) -> Result<(), Error> {
		let tcp_server = TcpListener::bind(address.into()).await?;
		let handler = task::spawn(HttpServer::tcp_handler(self, tcp_server));
		task::block_on(handler)
	}



}