use crate::routes::{health_check, subscribe};
use actix_web::{
   App,
   web,
   dev::Server,
   HttpServer,
   middleware::Logger,
};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;


pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
            App::new()
                // Middlewares are added using the `wrap` method on `App`
                .wrap(TracingLogger::default())
                .route("/health_check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscribe))
                .app_data(db_pool.clone())
        })
        .listen(listener)?
        .run();
    Ok(server)
}