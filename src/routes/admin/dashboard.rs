use crate::session_state::TypedSession;
use crate::utils::e500;
use actix_web::http::header::ContentType;
use actix_web::http::header::LOCATION;
use actix_web::web;
use actix_web::HttpResponse;
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn admin_dashboard(
    session: TypedSession,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = if let Some(user_id) = session.get_user_id().map_err(e500)? {
        get_username(user_id, &pool).await.map_err(e500)?
    } else {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish());
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta http-equiv="content-type" content="text/html; charset=utf-8 IE=edge">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Admin Dashboard</title>
        </head>
            <body>
                <P>Welcome {username}!</P>
                <p>Available options:</p>
                <ol>
                    <li><a href="/admin/password">Change Password</li>
                </ol>
            </body>
        </html>"#
        )))
}

#[tracing::instrument(name = "Get username", skip(pool))]
async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!("SELECT username FROM users WHERE user_id = $1", user_id)
        .fetch_one(pool)
        .await
        .context("Failed to perform a query to retrieve a username")?;

    Ok(row.username)
}
