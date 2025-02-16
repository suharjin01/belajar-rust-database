fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use sqlx::{Connection, Error, PgConnection};

    
    #[tokio::test]
    async fn test_manual_connection() -> Result<(), Error> {
        let url = "postgres://suharjin:@localhost:5432/belajar_rust";
        let connection: PgConnection = PgConnection::connect(url).await?;

        connection.close().await?;
        Ok(())
    }
}