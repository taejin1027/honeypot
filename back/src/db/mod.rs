use mongodb::options::ClientOptions;
use mongodb::bson::{doc, DateTime, Document};
use mongodb::{Client, Database};
use rocket::fairing::AdHoc;
use std::env;
use crate::model::User;

//pub mod customer;

//let MONGO_URI = "localhost:8081";
//let MONGO_DB_NAME = "honey";

pub fn init() -> AdHoc {
    AdHoc::on_ignite("Connecting to MongoDB", |rocket| async {
        match connect().await {
            Ok(database) => rocket.manage(database),
            Err(error) => {
                panic!("Cannot connect to instance:: {:?}", error)
            }
        }
    })
}

pub async fn connect() -> mongodb::error::Result<Database> {
    let mongo_uri = "mongodb://root:example@mongo:27017/";//env::var("MONGO_URI").expect("MONGO_URI is not found.");
    let mongo_db_name = "honey";//env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME is not found.");

    let client_options = ClientOptions::parse(mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database(mongo_db_name);//.as_str());

    println!("MongoDB Connected!");

    Ok(database)
}

pub async fn find_user_by_id(
    db: &Database,
    uid: &String,
    upw: &String,
) -> mongodb::error::Result<Option<User>> {
    println!("hellooooooooo ---- p0 ");
    let collection = db.collection::<User>("user");

    println!("hellooooooooo ---- p1 ");
    let user = collection.find_one(doc! {"id":uid }, None)?.await?;
    println!("hellooooooooo ---- p2 ");

    if user.is_none() {
        println!("hellooooooooo ---- p3 ");
        return Ok(None);
    }
    
    println!("hellooooooooo ---- p4 ");
    let user = user.unwrap();
    println!("hellooooooooo ---- p5 ");
    // transform ObjectId to String
    let user_json = User {
        _id: user._id.to_string(),
        id: user.id.to_string(),
        pw: user.pw.to_string(),
    };

    println!("helloooooooooo");
    println!("{}, {}, {}", user_json._id, user_json.id, user_json.pw);

    Ok(Some(user_json))
}