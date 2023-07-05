use std::collections::HashMap;
use reqwest::Error;
use serde_json::{Value, from_str, json};
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() -> Result<(), Error> {
    loop {

        let mut place_id = String::new();

        print!("> Enter a place id: ");

        stdout().flush().unwrap();

        stdin().read_line(&mut place_id).expect("failed to read input");
        
        let res = reqwest::get(format!("https://apis.roblox.com/universes/v1/places/{}/universe", place_id))
        .await?
        .json::<HashMap<String, u64>>()
        .await?;

        let universe_id = res.get("universeId").unwrap();

        let places = reqwest::get(format!("https://develop.roblox.com/v1/universes/{}/places?limit=100&sortOrder=Asc", universe_id))
        .await?
        .text()
        .await?;

        let parsed_data: Value = from_str(&places).unwrap();
        let data = parsed_data["data"].as_array().unwrap();
        let parsed_data: Value = from_str(&places).unwrap();
        
        let modified_data: Vec<Value> = parsed_data["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| 
        {
            let mut modified_item = item.clone();
            
            modified_item["description"] = json!("nobody reads descriptions 🤓");

            modified_item
            
        }).collect();

        println!("\n{}\n", serde_json::to_string_pretty(&modified_data).unwrap());
        println!("> successfully found {}", format!("{} places under {}", data.len(), place_id));
    }
}