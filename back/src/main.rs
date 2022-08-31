#[macro_use] extern crate rocket;

use crate::db::find_user_by_id;
use crate::model::User;

mod db; 
mod model;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    // .attach(db::init())
    .mount("/", routes![index, hello, login])
}


// fn test_option() {
//     let x = Some(5);
//     match test_option_sub(x) {
//         Some(5) => println!("six"),
//         _ => println!("else")
//     }
//     if let Some(6) = test_option_sub(x) {
//         println!("six");
//     }
//     else {
//         println!("else");
//     }
// }

#[get("/login?<id>&<pw>")]
async fn login(id: String, pw: String) -> String {
    //TODO implemen

    let database = db::connect().await.unwrap();
    
    println!("hellooooooooo ------ Main0 ");
    // mongodb::error::Result<Option<User>>

    let user_dummy = User {
        _id: "1".to_string(),
        id: "2".to_string(),
        pw: "3".to_string(),
    };
    let user_dummy2 = User {
        _id: "1".to_string(),
        id: "2".to_string(),
        pw: "3".to_string(),
    };


    let xxx = match find_user_by_id(&database, &id,&pw).await { 
        Ok(op) => op,
        Err(_) => Some(user_dummy)
        };

    let usr : User = match xxx {
        Some(yyy) => yyy,
        None => user_dummy2
    };
    println!("{}", usr.id);
    println!("hellooooooooo ------ Main1 ");
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