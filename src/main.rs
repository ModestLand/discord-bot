mod discord;
mod graphql;
mod postgres;
mod twitch;
mod utils;

use dotenv;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  pretty_env_logger::init();

  let pool = crate::postgres::connection::setup()
    .await
    .expect("Failed to setup postgres");

  crate::discord::connection::setup(pool.clone()).await;
  crate::graphql::server::start(pool.clone()).await;
}
