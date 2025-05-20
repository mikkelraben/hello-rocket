#[macro_use]
extern crate rocket;

use core::str;

use bson::{oid::ObjectId, Bson};
use rocket::{
    serde::{self, json::Json, Deserialize},
    State,
};

use ::serde::Serialize;
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

#[derive(Serialize, Deserialize, Debug)]
struct UserSettings {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    timeLimit: i32,
    colorWalk: [i32; 3],
    colorAlarm: [i32; 3],
    startTime: String,
    endTime: String,
}

struct Collections {
    users: Collection<Document>,
    measurements: Collection<Document>,
}

#[post("/", data = "<thing>")]
async fn post_index(repo: &State<Collections>, thing: Json<Thing<'_>>) -> () {
    repo.measurements
        .insert_one(
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
    let result = repo.users.find_one(Some(filter), None).await.unwrap();
    let docu = result.unwrap();

    let user_settings: Result<UserSettings, bson::de::Error> =
        bson::from_bson(bson::Bson::Document(docu));
    match user_settings {
        Ok(settings) => {
            let json = serde::json::to_string(&settings).unwrap();
            return json;
        }
        Err(_) => {
            return String::new();
        }
    }
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
