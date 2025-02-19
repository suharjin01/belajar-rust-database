fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use std::time::Duration;

    use futures::TryStreamExt;
    use sqlx::{postgres::{PgPoolOptions, PgRow}, Connection, Error, PgConnection, Pool, Postgres, Row};

    
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


    // Query SQL
    // - Fetch Optional
    #[tokio::test]
    async fn test_fetch_optional() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result = sqlx::query("select * from category where id = $1")
            .bind("A")
            .fetch_optional(&pool).await?;

        if let Some(row) = result {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");

            println!("id : {}, name : {}, description : {}", id, name, description)
        } else {
            println!("Data Is Not Found")
        }

        Ok(())
    }


    // - Fetch One
    #[tokio::test]
    async fn test_fetch_one() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result: PgRow = sqlx::query("select * from category where id = $1")
            .bind("B")
            .fetch_one(&pool).await?;

        let id: String = result.get("id");
        let name: String = result.get("name");
        let description: String = result.get("description");

        println!("id : {}, name : {}, description : {}", id, name, description);

        Ok(())
    }


    // - Fetch All
    #[tokio::test]
    async fn test_fetch_all() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result: Vec<PgRow> = sqlx::query("select * from category")
            .fetch_all(&pool).await?;

        for row in result {
            let id: String = row.get("id");
        let name: String = row.get("name");
        let description: String = row.get("description");

        println!("id : {}, name : {}, description : {}", id, name, description);
        }

        Ok(())
    }


    // - Fetch Stream
    #[tokio::test]
    async fn test_fetch() -> Result<(), Error> {
        let pool = get_pool().await?;

        let mut result = sqlx::query("select * from category")
            .fetch(&pool);

        while let Some(row) = result.try_next().await? {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");

            println!("id : {}, name : {}, description : {}", id, name, description);
        }

        Ok(())
    }
}