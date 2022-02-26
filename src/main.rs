use std::env;

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn greeting() -> impl Responder {
	String::from("Hello, world!")
}

#[get("/{name}")]
async fn greeting_name(web::Path(name): web::Path<String>) -> impl Responder {
	format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let mut port = String::from("3000");
	
	if let Ok(val) = env::var("PORT") {
		port = val;
	}

	println!("Running on 0.0.0.0:{port}");

	HttpServer::new(
		|| App::new()
			.service(greeting_name)
			.service(greeting)
	).bind(format!("0.0.0.0:{port}"))?
		.run()
		.await
}


