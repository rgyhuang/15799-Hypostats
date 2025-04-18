extern crate tide;
use tide::Request;
use tide::prelude::*;
use sqlx::PgPool;
// use hypostats::spi_return_query;

#[derive(Debug, Deserialize)]
struct StatsDump {
    starelid: i32,
    staattnum: i32,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let pool = PgPool::connect("postgres://ekwong@localhost:28813/hypostats").await?;
    println!("Connected to Postgres!");

    let mut app = tide::with_state(pool);
    app.at("/query").post(handle_stats);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn handle_stats(mut req: Request<PgPool>) -> tide::Result {
    let StatsDump { starelid, staattnum } = req.body_json().await?;
    let pool = req.state();

    let query = format!("SELECT spi_return_query({}, {})", starelid, staattnum);
    // This calls your Rust-defined SQL function inside the pgrx extension!
    let query_result: (String, ) = sqlx::query_as(&query)
        .fetch_one(pool)
        .await?;

    Ok(query_result.0.into()) // Return the result as text
}