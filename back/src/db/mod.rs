use mongodb::options::ClientOptions;
use mongodb::{Client, Database};
use rocket::fairing::AdHoc;
use std::env;

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

async fn connect() -> mongodb::error::Result<Database> {
    let mongo_uri = "mongodb://root:example@mongo:27017/";//env::var("MONGO_URI").expect("MONGO_URI is not found.");
    let mongo_db_name = "honey";//env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME is not found.");

    let client_options = ClientOptions::parse(mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database(mongo_db_name);//.as_str());

    println!("MongoDB Connected!");

    Ok(database)
}