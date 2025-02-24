use color_eyre::eyre::Report;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub async fn connect() -> Result<Pool<Postgres>, Report> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;
    Ok(pool)
}

pub async fn init_db(pool: &Pool<Postgres>) -> Result<(), Report> {
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

pub async fn populate_db(pool: &Pool<Postgres>) -> Result<(), Report> {
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

pub async fn query_db(pool: &Pool<Postgres>) -> Result<(), Report> {
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
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
