use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    
    println!("Mini Redis server listening on 127.0.0.1:6379 🚀");

    loop {
        // The second item contains the IP and port of the new connection.
        match listener.accept().await {
            Ok((socket, _)) => {
                if let Err(e) = process(socket).await {
                    eprintln!("Error processing connection: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

async fn process(socket: TcpStream) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await? {
        println!("Received frame: {:?}", frame);

        // Respond with an error
        let response = Frame::Error("command not implemented yet".to_string());
        connection.write_frame(&response).await?;
    }
    
    Ok(())
}