use color_eyre::eyre::Report;
#[allow(unused_imports)]
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tmptestpgci;

#[tokio::main]
async fn main() -> Result<(), Report> {
    let pool = tmptestpgci::connect().await?;
    println!("Hello, postgres: {:?}", pool);
    tmptestpgci::init_db(&pool).await?;
    tmptestpgci::populate_db(&pool).await?;
    tmptestpgci::query_db(&pool).await?;

    Ok(())
}

#[cfg(test)]
mod test {

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
