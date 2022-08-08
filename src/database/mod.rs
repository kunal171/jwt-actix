use mongodb::sync::{Client, Collection, Database};

use std::env;

pub fn get_db() -> Database {
    let database_name: String = env::var("MONGO_DB_NAME").unwrap();
    let db_conn_string: String = env::var("DB_CONN_STRING").unwrap();
    let client = Client::with_uri_str(&db_conn_string).unwrap();
    // let client_options =
    //     mongodb::options::ClientOptions::parse("mongodb://localhost:27017").unwrap();
    // let client = mongodb::sync::Client::with_options(client_options).unwrap();
    client.database(&database_name)
}

pub fn get_user_collection() -> Collection {
    get_db().collection("users")
}

