extern crate tide;
use http_types::headers::HeaderValue;
use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::fs::File;
use std::io::prelude::*;
use tide::security::{CorsMiddleware, Origin};
use tide::{prelude::*, Request};

#[derive(Debug, Deserialize)]
struct TableInfo {
    relname: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TableDump {
    class_info: String,
    stats_info: Vec<String>,
    atts_info: Vec<String>,
}

struct ClassInfo {
    oid: i32,
    relnatts: i16,
}

impl FromRow<'_, PgRow> for ClassInfo {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            oid: {
                let mut bytes: [u8; 4] = [0; 4];
                let row_bytes = row.try_get_raw(0)?.as_bytes().unwrap();
                bytes.copy_from_slice(&row_bytes[..4]);
                i32::from_be_bytes(<[u8; 4]>::try_from(bytes).unwrap())
            },
            relnatts: row.try_get(1)?,
        })
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: <username> <port>");
        return Ok(());
    }
    let username = args[1].clone();
    let port = args[2].clone();
    println!("Connecting to Postgres at port {}", port);
    println!("Using username {}", username);
    let url = format!("postgres://{}@localhost:{}/hypostats", username, port);
    let pool = PgPool::connect(&url).await?;
    println!("Connected to Postgres!");

    let mut app = tide::with_state(pool);
    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);

    app.with(cors);
    app.at("/explain").post(test);
    app.at("/export").post(table_dump);
    app.at("/export_dump").post(table_export);
    app.at("/load").post(table_load);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn test(req: Request<PgPool>) -> tide::Result {
    let pool = req.state();

    let query = format!("EXPLAIN SELECT * FROM BAR");
    let query_result: (String,) = sqlx::query_as(&query).fetch_one(pool).await?;

    Ok(query_result.0.into()) // Return the result as text
}

async fn table_export(mut req: Request<PgPool>) -> tide::Result {
    let TableDump {
        class_info,
        stats_info,
        atts_info,
    } = req.body_json().await?;

    let dump_str = serde_json::to_string(&TableDump {
        class_info,
        stats_info,
        atts_info,
    })
    .unwrap();

    // let pool: &sqlx::Pool<sqlx::Postgres> = req.state();

    let mut file = File::create("table_dump.json")?;
    file.write_all(dump_str.as_bytes())?;

    Ok("Successfully exported data\n".into())
}

async fn table_dump(mut req: Request<PgPool>) -> tide::Result {
    let TableInfo { relname } = req.body_json().await?;
    let pool = req.state();

    let query = format!(
        "SELECT oid, relnatts from pg_class where relname='{}'",
        relname
    );
    let ClassInfo { oid, relnatts } = sqlx::query_as(&query).fetch_one(pool).await?;
    println!("oid: {} num columns: {}\n", oid, relnatts);
    let class_query = format!("SELECT pg_class_dump({})", oid);
    let class_dump: (String,) = sqlx::query_as(&class_query).fetch_one(pool).await?;
    println!("Received class dump\n{}\n", class_dump.0);

    let mut stat_dumps: Vec<String> = Vec::with_capacity(relnatts as usize);
    for i in 1..relnatts + 1 {
        let stat_query = format!(
            "SELECT pg_statistic_dump({}, CAST ({} as SMALLINT))",
            oid, i
        );
        let stat_dump: (String,) = sqlx::query_as(&stat_query).fetch_one(pool).await?;
        println!(
            "Received dump for starelid {}, staattnum {}\n{}\n",
            oid, i, stat_dump.0
        );
        stat_dumps.push(stat_dump.0);
    }

    let mut att_dumps: Vec<String> = Vec::with_capacity(relnatts as usize);
    for i in 1..relnatts + 1 {
        let att_query = format!("SELECT pg_attribute_dump({}, {})", oid, i as i32);
        let att_dump: (String,) = sqlx::query_as(&att_query).fetch_one(pool).await?;
        println!(
            "Received dump for oid {}, attcol {}\n{}\n",
            oid, i, att_dump.0
        );
        att_dumps.push(att_dump.0);
    }

    let full_dump = TableDump {
        class_info: class_dump.0,
        stats_info: stat_dumps,
        atts_info: att_dumps,
    };
    let json_str = serde_json::to_string(&full_dump).unwrap();
    Ok(format!("{json_str}\n").into())
}

async fn table_load(mut req: Request<PgPool>) -> tide::Result {
    let TableDump {
        class_info,
        stats_info,
        atts_info,
    } = req.body_json().await?;
    let pool = req.state();

    let class_load_query = format!("SELECT pg_class_load('{}')", class_info);
    let class_loaded: (bool,) = sqlx::query_as(&class_load_query).fetch_one(pool).await?;
    if !class_loaded.0 {
        return Ok("Failed to load pg_class column\n".into());
    }

    for stat in stats_info.iter() {
        let stat_load_query = format!("SELECT pg_statistic_load('{}')", stat);
        let stat_loaded: (bool,) = sqlx::query_as(&stat_load_query).fetch_one(pool).await?;
        if !stat_loaded.0 {
            return Ok("Failed to a load pg_statistic\n".into());
        }
    }

    for att in atts_info.iter() {
        let att_load_query = format!("SELECT pg_attribute_load('{}')", att);
        let att_loaded: (bool,) = sqlx::query_as(&att_load_query).fetch_one(pool).await?;
        if !att_loaded.0 {
            return Ok("Failed to a load pg_attribute\n".into());
        }
    }
    println!("Successfully loaded data\n");
    Ok("Successfully loaded data\n".into())
}
