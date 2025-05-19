#[macro_use]
extern crate rocket;

use core::str;

use rocket::{
    serde::{self, json::Json, Deserialize},
    State,
};

use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

const CLIENT_URI: &str = "mongodb://username:password@localhost:27017/";

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Thing<'r> {
    device_id: &'r str,
    device_type: &'r str,
    measurement: bool,
}

struct Collections {
    users: Collection<Document>,
    measurements: Collection<Document>,
}


#[post("/", data = "<thing>")]
async fn post_index(repo: &State<Collections>, thing: Json<Thing<'_>>) -> () {
    repo.measurements.insert_one(
        doc! {
            "device_id": thing.device_id,
            "device_type": thing.device_type,
            "measurement": thing.measurement,
            "measure_time": chrono::Utc::now().to_rfc3339()
        },
        None,
    )
    .await
    .expect("Failed to insert document");
}

#[get("/settings", data = "<user>")]
async fn get_settings(repo: &State<Collections>, user: &str) -> String {
    let filter = doc! { "user": user };
    let result: Option<Document> = repo.users.find_one(filter, None).await.unwrap();
    match result {
        Some(doc) => {
            let json = serde::json::to_string(&doc).unwrap();
            return json;
        }
        None => {
            println!("No document found for user {}", user);
        }
    }
    String::new()
}

#[launch]
async fn rocket() -> _ {
    let client = Client::with_uri_str(CLIENT_URI).await.unwrap();

    let database = client.database("Lightguide");

    let users_collection: Collection<Document> = database.collection("Users");

    let measurements_collection: Collection<Document> = database.collection("Measurements");

    rocket::build()
        .manage(Collections {
            users: users_collection,
            measurements: measurements_collection,
        })
        .mount("/", routes![get_settings, post_index])
}
