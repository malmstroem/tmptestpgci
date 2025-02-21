use color_eyre::eyre::Report;
#[allow(unused_imports)]
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tmptestpgci;

#[tokio::main]
async fn main() -> Result<(), Report> {
    let pool = connect().await?;
    println!("Hello, postgres: {:?}", pool);

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

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_connect() {
        let _pool = connect().await.unwrap();
    }

    #[test]
    fn test_trivial() {
        let sum = 3 + 3;
        assert!(sum == 6);
    }

    #[test]
    #[should_panic]
    fn test_trivial_failure() {
        let sum = 3 + 3;
        assert!(sum == 7);
    }
}
