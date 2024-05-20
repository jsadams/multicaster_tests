// import Rocket
#[macro_use] extern crate rocket;

// add our routes and services modules
mod routes;
mod services;

// import our routes
use routes::date::get_current_date;
use routes::date::date_plus_month;

// this is our get route which will be requested at the "/" location wherever it is mounted
#[get("/")]
fn say_hello() -> &'static str {
    "Hello, welcome to the api!"
}

#[get("/")]
fn show_root() -> &'static str {
    "Hello, welcome to the api\n you probably want to go to /api"
}

// start the web server and mount our get route at "/api". Can replace /api with anything
// or just leave it as "/" as the default location
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![get_current_date, date_plus_month])
        .mount("/hello", routes![say_hello])
        .mount("/", routes![show_root])
}