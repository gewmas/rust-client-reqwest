use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct IP {
    origin: String,
}

async fn get() -> Result<(), Box<dyn std::error::Error>> {
    println!("=====GET======");
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<IP>()
        .await?;
    println!("{:#?}", resp);
    println!("{:#?}", resp.origin);
    Ok(())
}

async fn post() -> Result<(), Box<dyn std::error::Error>> {
    println!("=====POST======");
    // This will POST a body of `{"lang":"rust","body":"json"}`
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .json(&map)
        .send()
        .await?;
    println!("{:#?}", res);
    Ok(())
}

fn blocking() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://www.rust-lang.org")?.text()?;
    println!("get body = {:?}", body);

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()?;
    println!("post res = {:?}", res);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    get().await?;
    post().await?;
    Ok(())
}
