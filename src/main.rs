fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use std::time::Duration;

    use sqlx::{postgres::PgPoolOptions, Connection, Error, PgConnection, Pool, Postgres};

    
    #[tokio::test]
    async fn test_manual_connection() -> Result<(), Error> {
        let url = "postgres://suharjin:@localhost:5432/belajar_rust";
        let connection: PgConnection = PgConnection::connect(url).await?;

        connection.close().await?;
        Ok(())
    }


    // Database Pool
    #[tokio::test]
    async fn test_pool_connection() -> Result<(), Error> {
        let pool = get_pool().await?;
        pool.close().await;
        Ok(())
    }

    async fn get_pool() -> Result<Pool<Postgres>, Error> {
        let url = "postgres://suharjin:@localhost:5432/belajar_rust";
        PgPoolOptions::new()
            .max_connections(10)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(60))
            .connect(url).await
    }


    // Execute SQL
    // Mengisi tabel "category" di dalam database "belajar_rust"
    #[tokio::test]
    async fn test_execute() -> Result<(), Error> {
        let pool = get_pool().await?;
        sqlx::query("insert into category(id, name, description) values ('A', 'Aqil', 'Contoh');")
            .execute(&pool).await?;
        Ok(())
    }


    // Prepare Statement bisanya digunakan untuk menghindari SQL Injection
    // ketika menggunakan input user
    #[tokio::test]
    async fn test_prepare_statement() -> Result<(), Error> {
        let pool = get_pool().await?;
        sqlx::query("insert into category(id, name, description) values ($1, $2, $3);")
            .bind("B")
            .bind("Suharjin")
            .bind("Conto Description")
            .execute(&pool).await?;
        Ok(())
    }
}