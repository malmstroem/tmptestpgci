use color_eyre::eyre::Report;
#[allow(unused_imports)]
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tmptestpgci;

#[tokio::main]
async fn main() -> Result<(), Report> {
    let pool = connect().await?;
    println!("Hello, postgres: {:?}", pool);
    init_db(&pool).await?;
    populate_db(&pool).await?;
    query_db(&pool).await?;

    let _ = tmptestpgci::add(4, 5);
    Ok(())
}

async fn connect() -> Result<Pool<Postgres>, Report> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;
    Ok(pool)
}

async fn init_db(pool: &Pool<Postgres>) -> Result<(), Report> {
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS datatable (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        );
        "#
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn populate_db(pool: &Pool<Postgres>) -> Result<(), Report> {
    let name = String::from("name");
    let age = 30;
    sqlx::query!(
        "INSERT INTO datatable (name, age) VALUES ($1, $2)",
        name,
        age
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn query_db(pool: &Pool<Postgres>) -> Result<(), Report> {
    let name = String::from("name");
    let row = sqlx::query!("SELECT id,name,age FROM datatable WHERE name = $1", name)
        .fetch_optional(pool)
        .await?;

    if let Some(r) = row {
        println!("{} {} {}", r.id, r.name, r.age);
    } else {
        panic!("Cannot retrieve any data");
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_connect() {
        let _pool = connect().await.unwrap();
    }

    #[serial]
    #[tokio::test]
    async fn test_init_db() {
        let pool = connect().await.unwrap();
        init_db(&pool).await.unwrap();
    }

    #[serial]
    #[tokio::test]
    async fn test_populate_db() {
        let pool = connect().await.unwrap();
        populate_db(&pool).await.unwrap();
    }

    #[serial]
    #[tokio::test]
    async fn test_query_db() {
        let pool = connect().await.unwrap();
        query_db(&pool).await.unwrap();
    }

    #[serial]
    #[test]
    fn test_trivial() {
        let sum = 3 + 3;
        assert!(sum == 6);
    }

    #[serial]
    #[test]
    #[should_panic]
    fn test_trivial_failure() {
        let sum = 3 + 3;
        assert!(sum == 7);
    }
}
