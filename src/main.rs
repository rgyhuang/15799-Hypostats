extern crate tide;
use pgrx::Spi;
use tide::Request;
use tide::prelude::*;
// use hypostats::spi_return_query;

#[derive(Debug, Deserialize)]
struct Animal {
    query: String,
    legs: u16,
}

fn spi_return_query(_query: String) -> String {
    Spi::connect(|_client| {
        // let result = client.select(&query, None, &[]);
        // let mut output: String = String::new();

        // if let Ok(row) = result {
        //     let num_cols = row.columns().unwrap();
        //     let mut row_values = Vec::new();

        //     for col in 1..=num_cols {
        //         let value: Option<String> = row.get(col).unwrap_or(None);
        //         row_values.push(value.unwrap_or_else(|| "NULL".to_string()));
        //     }

        //     output = format!("{}\n{}", output, row_values.join(", "));
        // }
        // output
        "abcdef".to_ascii_lowercase()
    })
}

// #[async_std::main]
// async fn main() -> tide::Result<()> {
//     let mut app = tide::new();
//     app.at("/orders/shoes").post(order_shoes);
//     app.listen("127.0.0.1:8080").await?;
//     Ok(())
// }

fn main() {
  let result = spi_return_query("hello".to_ascii_lowercase());
  println!("{}", result);
  // println!("Hello");
}

// fn order_shoes(mut req: Request<()>) -> tide::Result {
//     let Animal { query, legs } = req.body_json();
//     let query_result: String = spi_return_query(query);
//     // let query_result = query;
//     Ok(format!("Result is: {}, legs: {}\n", query_result, legs).into())
// }

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { query, legs } = req.body_json().await?;
    let query_result: String = spi_return_query(query);
    // let query_result = query;
    Ok(format!("Result is: {}, legs: {}\n", query_result, legs).into())
}