#[macro_use] extern crate rocket;

mod db; 

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(db::init())
    .mount("/", routes![index, hello, login])
}

#[get("/login?<id>&<pw>")]
fn login(id: String, pw: String) -> String {
    if id == "abc" && pw == "1234" {
        format!("Hello, {}!", id.as_str())
    }
    else
    {
        format!("Get a way from here")
    }
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}