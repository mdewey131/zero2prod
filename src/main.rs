use std::net::TcpListener;
use sqlx::{
    PgPool,
    postgres::PgPoolOptions,
};
use secrecy::ExposeSecret;
use zero2prod::{
    startup::run,
    configuration::get_configuration,
    telemetry::{get_subscriber, init_subscriber}
};



#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    
    let subscriber = get_subscriber("zero_2_prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read config");
    let connection_pool = PgPoolOptions::new() 
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(
            &configuration.database.connection_string().expose_secret()
        )
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
