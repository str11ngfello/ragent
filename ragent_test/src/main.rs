#![allow(dead_code)]
#![allow(unused_variables)]
mod adder;
use adder::{Adder, AdderArgs};
use ragent::tool::Tool;
use tokio;

#[tokio::main]
async fn main() {
    match (Adder {}).run(AdderArgs { x: 10.0, y: 2.0 }) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
