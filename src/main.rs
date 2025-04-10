#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize};

use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

const client_uri: &str = "mongodb://username:password@localhost:27017/";
let users_collection: Collection<Document>

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Thing<'r> {
    device_id: &'r str,
    device_type: &'r str,
    measurement: bool,
}

#[post("/", data = "<thing>")]
fn post_index(thing: Json<Thing<'_>>) -> () {
    users_collection.
        insert_one(
            doc! {
                "device_id": thing.device_id,
                "device_type": thing.device_type,
                "measurement": thing.measurement,
            },
            None,
        )
        .expect("Failed to insert document");
}

#[launch]
fn rocket() -> _ {
    let client = Client::with_uri_str(client_uri);

    let database = client.database("Lightguide");
    users_collection = database.collection("Users");

    rocket::build().mount("/", routes![index, post_index])
}
