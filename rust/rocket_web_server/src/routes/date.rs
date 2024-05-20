use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};

// import services module
use crate::services;


//The HTTP specifications technically define the difference between "GET" and "POST" so that former means that form data is to be encoded (by a browser) into a URL while the latter means that the form data is to appear within a message body. But the specifications also give the usage recommendation that the "GET" method should be used when the form processing is "idempotent" (i.e., it has no lasting observable effect on the state of the world), and in those cases only. As a simplification, we might say that "GET" is basically for just getting (retrieving) data whereas "POST" may involve anything, like storing or updating data, or ordering a product, or sending E-mail.

// create a struct to hold our Date data
// need serialize/deserialize to convert to/from JSON
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: i32
}


// create get-current-date route under /date and call get_current_date service which will return a Date object
// route returns a Date object converted to JSON
#[get("/date/get-current-date")]
pub fn get_current_date() -> Json<Date> {
    Json(services::date::get_current_date())
}

// route will accept data in JSON format and expects a date variable in the function parameters
#[post("/date/date-plus-month", format = "json", data = "<date>")]
pub fn date_plus_month(date: Json<Date>) -> Json<Date> {
    Json(services::date::date_plus_month(date))
}