use mongodb::Client;
use std::error::Error;

pub struct ClientConnection {
    connection: Client,
}

impl ClientConnection {
    pub async fn new(connection_str: &str) -> Result<Self, Box<dyn Error>> {
        let connection = Client::with_uri_str(connection_str).await?;
        Ok(ClientConnection { connection })
    }
}
