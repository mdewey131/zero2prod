use std::net::TcpListener;
use sqlx::{
    PgPool
};
use env_logger::Env;
use zero2prod::{
    startup::run,
    configuration::get_configuration 
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // `init` does call `set_logger`, so this is all we need to do. 
    // We are falling back to printing all logs at info-level or above
    // If the `RUST_LOG` environment variable has not been set
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read config");
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    )
        .await 
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
