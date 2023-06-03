use std::{env, io};

pub async fn connect_mongodb() -> Result<mongodb::Database, io::Error> {
    let db_name: String = env::var("MONGODB_DB_NAME").unwrap();

    /* Load the MongoDB connection string from an environment variable */
    let client_uri =
        env::var("MONGODB_CONNECTION_URI").expect("You must set the MONGODB_URI environment var!");

    /* connect mongodb */
    let client = mongodb::Client::with_uri_str(client_uri)
        .await
        .expect("mongodb connection failed");

    /* pass used database */
    Ok(client.database(&db_name))
}
