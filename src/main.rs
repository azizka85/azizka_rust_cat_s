use std::env;
use std::time::{ SystemTime, UNIX_EPOCH };

use tide::{Request, Result, Body};
use tide::prelude::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct User {
	id: Option<u128>,
	name: String,
	email: String,
	password: String
}

async fn greeting(_: Request<()>) -> Result {
	Ok(String::from("Hello, world!").into())
}

async fn greeting_name(req: Request<()>) -> Result {
	Ok(format!("Hello {}!", req.param("name").unwrap()).into())
}

async fn post_user(mut req: Request<()>) -> Result<Body> {
	let user = req.body_json().await?;

	Body::from_json(&User {		
		id: Some(SystemTime::now()
					.duration_since(UNIX_EPOCH)
					.expect("Time went backwards")
					.as_millis()),
		..user
	})
}

#[async_std::main]
async fn main() -> tide::Result<()> {
	let port = env::var("PORT").unwrap_or(String::from("3000"));

	println!("Running on 0.0.0.0:{port}");

	let mut app = tide::new();

	app.at("/").get(greeting);
	app.at("/post").post(post_user);
	app.at("/:name").get(greeting_name);	

	app.listen(format!("0.0.0.0:{port}")).await?;

	Ok(())
}


