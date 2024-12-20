use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> HttpResponse {
    // Lets generate a random unique identifier
    let request_id = Uuid::new_v4();

    log::info!(
        "request_id {}-, Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name
    );

    log::info!(
        " request_id {} - Saving new subscriber details in the database",
        request_id
    );

    // `Result` has two variants: `Ok` and `Err`
    // The first for success, the second for failures.
    // We use a `match` statement to choose what to do based
    // on the outcome
    // We will talk more about `Result` going forward

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => {
            log::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
