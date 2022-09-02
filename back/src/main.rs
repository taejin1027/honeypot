#[macro_use] extern crate rocket;

mod db;
mod model;

use crate::db::find_user;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, hello, login])
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