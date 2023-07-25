use std::net::TcpListener;
use sqlx::{
    PgPool
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
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string().expose_secret()
    )
        .await 
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
