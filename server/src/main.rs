mod dnguyen;

use rocket::{get, routes};

use tokio;
use tokio_postgres::{NoTls};

use dnguyen::error::{DBError};

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

