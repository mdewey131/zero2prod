use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    // Spans, like logs, have an associated level
    // `info_span` creates a span at the info level
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    // Using 'enter' in an async function is a recipe for disaster. So, don't do this
    // in production
    let _request_span_guard = request_span.enter();
    // request_span_guard is dropped at the end of `subscribe`, when we 'exit' the span
    
    // We do not call `.enter` on query_span!
    // .instrument takes care of it at the right moments 
    // in the query future lifetime
    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database"
    );
    match sqlx::query!(r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
     )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await 
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
