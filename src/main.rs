use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct IP {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<IP>()
        .await?;
    println!("{:#?}", resp);
    println!("{:#?}", resp.origin);
    Ok(())
}
