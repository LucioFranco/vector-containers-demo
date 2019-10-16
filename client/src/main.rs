use hyper::Client;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let uri = hyper::Uri::from_static("http://192.168.99.101:32655/");

    loop {
        client.get(uri.clone()).await?;
        tokio::timer::delay_for(std::time::Duration::from_millis(500)).await;
    }
}
