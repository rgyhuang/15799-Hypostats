extern crate tide;
use http_types::headers::HeaderValue;
use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::fs::File;
use std::io::prelude::*;
use tide::http::Response;
use tide::http::StatusCode;
use tide::security::{CorsMiddleware, Origin};
use tide::{prelude::*, Request};

#[derive(Debug, Deserialize)]
struct TableInfo {
    relname: String,
}
#[derive(Debug, Deserialize)]
struct ExplainQuery {
    query: String,
}

#[derive(Serialize)]
struct ExplainPlan {
    plan: Vec<String>,
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
                i32::from_be_bytes(bytes)
            },
            relnatts: row.try_get(1)?,
        })
    }
}

const DEBUG: bool = true;

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
    app.at("/explain").post(explain);
    app.at("/export").post(table_dump);
    app.at("/export_dump").post(table_export);
    app.at("/load").post(table_load);

    app.listen("127.0.0.1:8080").await?;
    println!("end");

    Ok(())
}

async fn explain(mut req: Request<PgPool>) -> tide::Result {
    let ExplainQuery { query } = req.body_json().await?;
    let pool = req.state();

    if DEBUG {
      println!("Received query: {}\n", query);
    }
    // collect all rows
    let stmt = format!("EXPLAIN {}", query);
    let rows = sqlx::query(&stmt).fetch_all(pool).await.map_err(|e| {
        println!("Error is {}", e);
        tide::Error::from_str(StatusCode::InternalServerError, format!("SQL error: {}", e))
    })?;

    // collect all rows into a vector of strings
    let plan_lines: Vec<String> = rows
        .into_iter()
        .map(|row| row.get::<String, _>(0))
        .collect();
    let resp_body = ExplainPlan { plan: plan_lines };

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(tide::Body::from_json(&resp_body)?);
    res.set_content_type(tide::http::mime::JSON);
    Ok(res.into())
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
    if DEBUG {
      println!("oid: {} num columns: {}\n", oid, relnatts);
    }
    let class_query = format!("SELECT pg_class_dump({})", oid);
    let class_dump: (String,) = sqlx::query_as(&class_query).fetch_one(pool).await?;
    if DEBUG {
      println!("Received class dump\n{}\n", class_dump.0);
    }

    let mut stat_dumps: Vec<String> = Vec::with_capacity(relnatts as usize);
    for i in 1..relnatts + 1 {
        let stat_query = format!(
            "SELECT pg_statistic_dump({}, CAST ({} as SMALLINT))",
            oid, i
        );
        let stat_dump: (String,) = sqlx::query_as(&stat_query).fetch_one(pool).await?;
        if DEBUG {
          println!(
            "Received dump for starelid {}, staattnum {}\n{}\n",
            oid, i, stat_dump.0
          );
        }
        stat_dumps.push(stat_dump.0);
    }

    let mut att_dumps: Vec<String> = Vec::with_capacity(relnatts as usize);
    for i in 1..relnatts + 1 {
        let att_query = format!("SELECT pg_attribute_dump({}, {})", oid, i as i32);
        let att_dump: (String,) = sqlx::query_as(&att_query).fetch_one(pool).await?;
        if DEBUG {
          println!(
            "Received dump for oid {}, attcol {}\n{}\n",
            oid, i, att_dump.0
          );
        }
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

    let pg_class_row: hypostats::PgClassRow = serde_json::from_str(&class_info).unwrap();
    let relname: String = pg_class_row.relname;
    let class_query = format!("SELECT oid, relnatts FROM pg_class WHERE relname='{}'", relname);
    let class_opt: Option<ClassInfo> = sqlx::query_as(&class_query).fetch_optional(pool).await?;
    if class_opt.is_none() {
      let create_query = format!("CREATE TABLE {}", relname);
      // Create table with all columns using atts_info
          // Will have to use attname and atttypid to figure out what type it should be
          // (VARCHAR, INT, DATE, etc)
      // Replace all instances of the OID (oid in class, starelid in pg_statistic, and 
      //   attrelid in pg_attribute)
    } else {
      // Make sure the OIDs match (if not, change those to match as well)
    }

    let class_load_query = format!("SELECT pg_class_load('{}')", class_info);
    let class_loaded: (bool,) = sqlx::query_as(&class_load_query).fetch_one(pool).await?;
    if !class_loaded.0 {
        return Ok("Failed to load pg_class column\n".into());
    }
    if DEBUG {
      println!("Successfully loaded pg_class");
    }

    for stat in stats_info.iter() {
        let stat_load_query = format!("SELECT pg_statistic_load('{}')", stat);
        if DEBUG {
          println!("stat is {}", stat);
        }
        let stat_loaded: (bool,) = sqlx::query_as(&stat_load_query).fetch_one(pool).await?;
        if !stat_loaded.0 {
            if DEBUG {
              println!("Failing at this stat");
            }
            return Ok("Failed to a load pg_statistic\n".into());
        }
    }
    if DEBUG {
      println!("Successfully loaded all statistics");
    }

    for att in atts_info.iter() {
        let att_load_query = format!("SELECT pg_attribute_load('{}')", att);
        let att_loaded: (bool,) = sqlx::query_as(&att_load_query).fetch_one(pool).await?;
        if !att_loaded.0 {
            return Ok("Failed to a load pg_attribute\n".into());
        }
    }
    if DEBUG {
      println!("Successfully loaded all attribute stats, finished with data!");
    }
    Ok("Successfully loaded data\n".into())
}
