use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::config::Config;

pub async fn create_connection(config: &Config) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
}

#[derive(sqlx::FromRow, Debug)]
struct City {
    name: String,
}

#[derive(Debug)]
struct High {
    city: Option<String>,
    high: Option<i32>,
}

pub async fn example() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://localhost/mydb")
        .await
        .unwrap();

    let row: (String,) = sqlx::query_as("SELECT city FROM weather")
        // .bind(String::new())
        .fetch_one(&pool)
        .await
        .unwrap();
    println!("{row:?}");

    let city = sqlx::query_as::<_, City>("SELECT name FROM cities")
        .fetch_one(&pool)
        .await
        .unwrap();
    println!("{city:?}");

    const HIGH: i32 = 50;
    let city_highs = sqlx::query_as!(
        High,
        "SELECT city, temp_hi AS high FROM weather WHERE temp_hi < $1",
        HIGH
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    println!("{city_highs:?}");
}
