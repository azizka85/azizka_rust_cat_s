use std::env;
use std::time::{ SystemTime, UNIX_EPOCH };
use std::collections::HashMap;

use tide::{Request, Response, Result, Body};
use tide::prelude::{Deserialize, Serialize};
use tide::http::mime::HTML;

use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "hello.html")]
struct HelloTemplate {
	name: Option<String>,
	tr: fn(key: &String, dic: &HashMap<String, String>) -> String
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
	id: Option<u128>,
	name: String,
	email: String,
	password: String
}

fn tr(key: &String, dic: &HashMap<String, String>) -> String {
	dic.get(key).unwrap_or(key).clone()
}

async fn greeting(_: Request<()>) -> Result<Response> {
	let mut res = Response::new(200);
	res.set_content_type(HTML);
	res.set_body(
		Body::from_string(
			HelloTemplate { 
				name: None,
				tr: tr 
			}
			.render_once()
			.unwrap()
		)
	);
	
	Ok(res)
}

async fn greeting_name(req: Request<()>) -> Result<Response> {
	let mut res = Response::new(200);
	res.set_content_type(HTML);
	res.set_body(
		Body::from_string(
			HelloTemplate { 
				name: Some(req.param("name")
								.unwrap()
								.into()),
				tr: tr 
			}
			.render_once()
			.unwrap()
		)
	);
	
	Ok(res)
}

async fn post_user(mut req: Request<()>) -> Result<Response> {
	let user = req.body_json().await?;

	let mut res = Response::new(201);	
	res.set_body(
		Body::from_json(&User {		
			id: Some(SystemTime::now()
						.duration_since(UNIX_EPOCH)
						.expect("Time went backwards")
						.as_millis()),
			..user
		}).unwrap()
	);

	Ok(res)
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


