use pea_api::get;
use std::error::Error;
const API: &str = "http://localhost:8080";
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let height = get::height(API).await?;
    let hash = get::hash(API, &height).await?;
    let block = get::block(API, &hash).await?;
    println!("{:?}", block);
    Ok(())
}
