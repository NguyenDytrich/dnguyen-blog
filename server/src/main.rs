use std::error::Error;
use std::fmt;

use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::http::Status;

use rocket::{get, routes};

use tokio;
use tokio_postgres::{NoTls};

#[derive(Debug)]
struct DBError;

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DBError.")
    }
}
impl Error for DBError {}
impl<'r> Responder<'r, 'r> for DBError {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        return Response::build()
            .status(Status::BadRequest)
            .ok();
    }
}
impl From<tokio_postgres::Error> for DBError {
    fn from(_: tokio_postgres::Error) -> Self {
        return DBError {};
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/db_test")]
async fn db_test() -> Result<String, DBError> {
    let (client, connection) = tokio_postgres::connect("host=localhost user=dytrich", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT * FROM messages", &[])
        .await?;

    let value: String = rows[0].get(0);

    return Ok(value);
}

#[rocket::main]
async fn main() {
    let _server = rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![db_test])
        .launch()
        .await;
}

