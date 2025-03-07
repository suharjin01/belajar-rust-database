fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chrono::{Local, NaiveDateTime};
    use futures::TryStreamExt;
    use sqlx::{postgres::{PgPoolOptions, PgRow}, prelude::FromRow, Connection, Error, PgConnection, Pool, Postgres, Row};

    
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


    // Result Mapping 
    #[derive(Debug, FromRow)]
    struct Category {
        id: String,
        name: String,
        description: String
    }

    // Result Mapping 
    // - Cara manual
    #[tokio::test]
    async fn test_result_mapping() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result: Vec<Category> = sqlx::query("select * from category")
            .map(|row: PgRow| {
                Category {
                    id: row.get("id"),
                    name: row.get("name"),
                    description: row.get("description")
                }
            })
            .fetch_all(&pool).await?;

        for category in result {
            println!("{:?}", category);
        }

        Ok(())
    }


    // Result Mapping 
    // - Cara otomatis
    #[tokio::test]
    async fn test_result_mapping_automatic() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result: Vec<Category> = sqlx::query_as("select * from category")
            .fetch_all(&pool).await?;

        for category in result {
            println!("{:?}", category);
        }

        Ok(())
    }


    // Data Type
    #[tokio::test]
    async fn test_insert_data_title() -> Result<(), Error> {
        let pool = get_pool().await?;

        sqlx::query("insert into title(id, title, description, created_at, updated_at) values ($1, $2, $3, $4, $5);")
            .bind("A")
            .bind("Borrowing")
            .bind("Memahami konsep borrowing di Rust")
            .bind(Local::now().naive_local())
            .bind(Local::now().naive_local())
            .execute(&pool).await?;

        Ok(())
    }

    // Membuat struct title
    #[derive(Debug, FromRow)]
    struct Title {
        id: String,
        title: String,
        description: String,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    }

    #[tokio::test]
    async fn test_result_mapping_title() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result: Vec<Title> = sqlx::query_as("select * from title")
            .fetch_all(&pool).await?;

        for title in result {
            println!("{:?}", title);
        }

        Ok(())
    }


    // Transaction
    #[tokio::test]
    async fn test_transaction() -> Result<(), Error> {
        let pool = get_pool().await?;
        
        let mut transaction = pool.begin().await?;

        sqlx::query("insert into title(id, title, description, created_at, updated_at) values ($1, $2, $3, $4, $5);")
            .bind("B")
            .bind("Memory Management")
            .bind("Konsep memory manajemen di Rust")
            .bind(Local::now().naive_local())
            .bind(Local::now().naive_local())
            .execute(&mut *transaction).await?;

        sqlx::query("insert into title(id, title, description, created_at, updated_at) values ($1, $2, $3, $4, $5);")
            .bind("C")
            .bind("Ownership")
            .bind("Konsep ownership di Rust")
            .bind(Local::now().naive_local())
            .bind(Local::now().naive_local())
            .execute(&mut *transaction).await?;

        transaction.commit().await?;

        Ok(())
    }


    // Auto Increment
    #[tokio::test]
    async fn test_auto_increment() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result: PgRow = sqlx::query("insert into mentors(name) values ($1) returning id;")
            .bind("Suharjin S.T")
            .fetch_one(&pool).await?;

        let id: i32 = result.get("id");
        println!("Id Mentor : {}", id);

        Ok(())
    }

    // Auto Increment (2)
    #[tokio::test]
    async fn test_auto_increment_with_transaction() -> Result<(), Error> {
        let pool = get_pool().await?;
        let mut transaction = pool.begin().await?;

        sqlx::query("insert into mentors(name) values ($1) returning id;")
            .bind("Suharjin S.T")
            .execute(&mut *transaction).await?;


        let result: PgRow = sqlx::query("select lastval() as id")
            .fetch_one(&mut *transaction).await?;

        let id: i32 = result.get_unchecked("id");
        println!("Id Mentor : {}", id);

        Ok(())
    }
}