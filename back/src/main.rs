#[macro_use] extern crate rocket;

mod db;
mod model;

use crate::db::find_user;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn index() -> &'static str {
    "hi, honeypot!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, hello, login])
    .attach(CORS)
}


#[get("/login?<id>&<pw>")]
async fn login(id: String, pw: String) -> String {
    let msg = match find_user(&id, &pw).await {
        Ok(_) => String::from("welcome!"),
        _ => String::from("get out!")
    };
    msg
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}