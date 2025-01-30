use rocket::{form::Form, launch, routes};

#[macro_use]
extern crate rocket;

#[derive(FromForm)]
struct HelloWorldForm {
    token: String
}

#[post("/hello_world", data = "<input>")]
fn hello_world(input: Form<HelloWorldForm>) -> String {
    input.token.clone()
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![]).configure(rocket::Config {
        address: "0.0.0.0".parse().unwrap(),
        port: 8814,
        ..Default::default()
    })
}
