use mongodb::Client;
use mongodb::options::ClientOptions;
use mongodb::bson::doc;
use futures::stream::TryStreamExt;
use crate::model::User;
use std::env;

// https://doc.rust-lang.org/std/result/enum.Result.html
// https://docs.rs/mongodb/latest/mongodb/error/type.Result.html
// https://docs.rs/mongodb/latest/mongodb/error/struct.Error.html
// https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html
// https://docs.rs/mongodb/latest/mongodb/error/enum.ErrorKind.html
// https://www.mongodb.com/docs/drivers/rust/
// https://docs.rs/mongodb/latest/mongodb/
pub async fn find_user(id: &String, pw: &String) -> mongodb::error::Result<String> {
    env::set_var("RUST_BACKTRACE", "1");

    let options = ClientOptions::parse("mongodb://honeypot:honeypot@host.docker.internal:27017/?authSource=admin").await?;
    let client = Client::with_options(options)?;
    for database_name in client.list_database_names(None, None).await? {
        println!("{}", database_name);
    }
    let database = client.database("honeypot");
    for collection_name in database.list_collection_names(None).await? {
        println!("{}", collection_name);
    }

    let user_collection = database.collection::<User>("user");
    let mut cursor = user_collection.find(doc! { "id": id }, None).await?;
    // Iterate over the results of the cursor.
    while let Some(user) = cursor.try_next().await? {
        println!("id: {}", user.id);
        return Ok(String::from("found"));
    }
    Err(mongodb::error::ErrorKind::SessionsNotSupported.into())
}
