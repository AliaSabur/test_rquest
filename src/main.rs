use rquest::{Client, Impersonate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build a client to impersonate Firefox135
    let client = Client::builder()
        .impersonate(Impersonate::Firefox135)
        .build()?;

    // Use the API you're already familiar with
    let resp = client.get("https://tls.peet.ws/api/all").send().await?;
    println!("{}", resp.text().await?);

    Ok(())
}
