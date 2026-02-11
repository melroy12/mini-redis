use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to the Redis server
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("Got value from Redis: {:?}", result);

    Ok(())
}
