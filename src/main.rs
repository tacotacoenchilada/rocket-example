#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] 
extern crate rocket;
use serde::{Deserialize, Serialize};
extern crate rocket_contrib;
use serde_json::*;
use rocket_contrib::json::Json;

#[derive(Deserialize, Serialize)]

struct Message {
    message: String
}

#[get("/")]
fn index() -> Json<Message> {
      let message = Message {
         message: "shit".to_string()
      };
     Json(message)
}

fn main() {
               rocket::ignite().mount("/shit", routes![index]).launch();
}
