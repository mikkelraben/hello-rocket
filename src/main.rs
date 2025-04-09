#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize};

#[get("/")]
fn index() -> &'static str {
    println!("Request received");

    "Hello, world!"
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Thing<'r> {
    device_id: &'r str,
    device_type: &'r str,
    measurement: bool,
}

#[post("/", data = "<thing>")]
fn post_index(thing: Json<Thing<'_>>) -> () {
    println!("POST request received");
    println!("device_id: {}", thing.device_id);
    println!("device_type: {}", thing.device_type);
    println!("measurement: {}", thing.measurement);
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, post_index])
}
