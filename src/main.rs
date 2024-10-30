use rquest;
use tokio;

#[tokio::main]
async fn main() {
    let url = "https://www.google.com";
    let client = rquest::Client::new();
    let res = client.get(url).send().await.unwrap();
    println!("Status: {}", res.status());

    println!("Hello, world!");
}
