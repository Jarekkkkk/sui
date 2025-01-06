pub mod schema;

use std::{env, time::Duration};
use diesel_async::{pooled_connection::{bb8::Pool, AsyncDieselConnectionManager, ManagerConfig}, AsyncPgConnection};
use dotenvy::dotenv;

pub type PgConnectionPool =
    diesel_async::pooled_connection::bb8::Pool<diesel_async::AsyncPgConnection>;
pub type PgPoolConnection<'a> =
    diesel_async::pooled_connection::bb8::PooledConnection<'a, AsyncPgConnection>;

pub async fn get_connection_pool() -> PgConnectionPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut config = ManagerConfig::default();
    config.custom_setup = Box::new(establish_connection);

    let manager = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new_with_config(
        database_url,
        config,
    );

    Pool::builder()
        .connection_timeout(Duration::from_secs(30))
        .build(manager)
        .await
        .expect("Could not build Postgres DB connection pool")
}
