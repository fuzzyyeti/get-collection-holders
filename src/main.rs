//noinspection RsMainFunctionNotFound
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{to_string_pretty, Value};
use std::collections::HashMap;
use std::io::{Read, Write};
use dotenv::dotenv;

const COLLECTION: &str = "J1S9H3QjnRtBbbuD4HjPV6RpRhwuk4zKbxsnCHuTgh9w"; //Mad Lads Collection Address

async fn get_page(page: u32) -> Result<(HashMap<String, u32>, u32), reqwest::Error> {
    let mut headers = HeaderMap::new();
    let shyft_api_key = dotenv::var("SHYFT_API_KEY").unwrap();
    headers.insert("x-api-key", HeaderValue::from_str(&shyft_api_key).unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let res = client.get(format!("https://api.shyft.to/sol/v1/collections/get_nfts?network=mainnet-beta&collection_address={}&page={}&size=50", COLLECTION, page))
        .headers(headers)
        .send()
        .await?;

    let mut holder_map: HashMap<String, u32> = HashMap::new();
    // Process the response
    let mut last_page = 0;
    if res.status().is_success() {
        let response: HashMap<String, Value> = res.json().await.unwrap();
        let result = response.get("result").unwrap();
        last_page = result.get("total_pages").unwrap().as_u64().unwrap() as u32;
        if let Value::Array(nfts) = result.get("nfts").unwrap() {
            for nft in nfts {
                let holder = nft.get("owner").unwrap().as_str().unwrap();
                holder_map.insert(holder.to_string(), holder_map.get(holder).unwrap_or(&0) + 1);
            }
        }
        for (key, value) in holder_map.iter() {
            println!("{}: {}", key, value);
        }
        println!("{} Holders in page {}", holder_map.keys().len(), page);
    } else {
        println!("Failed to make the request");
    }
    Ok((holder_map, last_page))
}

async fn create_holder_list() {
    let mut page = 1;
    let mut holder_map: HashMap<String, u32> = HashMap::new();
    loop {
        match get_page(page).await {
            Ok((mut map, last_page)) => {
                if map.is_empty() {
                    break;
                }
                for(key, value) in holder_map.iter() {
                    map.entry(key.to_string()).and_modify(|e| *e += value);
                }
                holder_map.extend(map);
                if page == last_page {
                    break;
                }
                page += 1;
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
    //write map to file
    let mut file = std::fs::File::create("holder_map.json").unwrap();
    file.write_all(to_string_pretty(&holder_map).unwrap().as_bytes()).unwrap();
}


//Check total number of NFTs in collection
fn check_nft_count() {
    let mut file = std::fs::File::open("holder_map.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let madlads = serde_json::from_str::<HashMap<String, u32>>(&data).unwrap();
    println!("{} total NFTs", madlads.values().sum::<u32>());
    println!("{} total Holders", madlads.keys().len());
    let mut top_holder = ("", 0);
    madlads.iter().for_each(|(address, holder)| {
        if holder > &top_holder.1 {
            top_holder = (address, *holder);
        }
    });
    println!("{} is the top holder with {} NFTs", top_holder.0, top_holder.1);
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    create_holder_list().await;
    println!("Holders List Created at ./holder_map.json");
    check_nft_count()
}
