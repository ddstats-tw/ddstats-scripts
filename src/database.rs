use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Pool, Postgres,
};
use std::{env, str::FromStr};

pub async fn init_db() -> Pool<Postgres> {
    let connect_options =
        PgConnectOptions::from_str(&env::var("DATABASE_URL").expect("failed to get DATABASE_URL"))
            .unwrap()
            .disable_statement_logging();

    PgPoolOptions::new()
        .max_connections(
            env::var("DATABASE_MAX_CONNECTIONS")
                .map(|x| x.parse().expect("not a number"))
                .unwrap_or(50),
        )
        .min_connections(
            env::var("DATABASE_MIN_CONNECTIONS")
                .map(|x| x.parse().expect("not a number"))
                .unwrap_or(1),
        )
        .connect_with(connect_options)
        .await
        .expect("could not connect to database")
}
