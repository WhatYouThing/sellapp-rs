# Sell.App API Wrapper for Rust

Simple asynchronous wrapper for the Sell.App API, using the ``reqwest`` crate under the hood.

## Example Usage

```rust
use sellapp;
use serde_json::{self, Value};
use tokio;

#[tokio::main]
async fn main() {
    let api = sellapp::init("your_api_key");

    let res = api.invoices_list_all("?limit=25").await.unwrap();

    let text = res.text().await;
    let data: Value = serde_json::from_str(&text.unwrap()).unwrap();
    println!("The ID of the 4th invoice is: {}", data["data"][3]["id"]);
}
```