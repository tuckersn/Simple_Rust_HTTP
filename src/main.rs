use http::server::HttpServer;
mod http;
mod util;


#[async_std::main]
async fn main() -> std::io::Result<()> {
	let mut app = HttpServer::new();

	app.register("/", |_, mut res| async move {
		res.status(200).body("Hello World!").send().await
	});

	app.register("/object/{id}/edit", |mut _req, mut res| async move {
		res.status(200).body(format!("ABC {}", _req.params["id"])).send().await
	});

	app.register("/hello", |_, mut res| async move {
		res.status(200)
			.send_html("<html><body><h1>Hello World</h1></body></html>").await
	});

	app.register("/ping", |mut _req, mut res| async move {

		let content = String::from_utf8(_req.body).unwrap();

		res.status(200)
			.send_html(format!("<html>
				<body>
					<h1>Hello World</h1>
					<p style:b
				</body>
			</html>")).await
	});


	app.register("/ping_raw", |mut _req, mut res| async move {
		res.status(200)
			.send_json(&_req).await
	});

	println!("Starting HTTP server");
	app.start("127.0.0.1:8080").await
}

