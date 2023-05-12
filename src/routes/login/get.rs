use crate::startup::HmacSecret;
use actix_web::cookie::{time::Duration, Cookie};
use actix_web::http::header::ContentType;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use hmac::{Hmac, Mac};
use secrecy::ExposeSecret;
use std::fmt::Write;

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: String,
    tag: String,
}

impl QueryParams {
    fn verify(self, secret: &HmacSecret) -> Result<String, anyhow::Error> {
        let tag = hex::decode(self.tag)?;
        let query_string = format!("error={}", urlencoding::Encoded::new(&self.error));

        let mut mac =
            Hmac::<sha2::Sha256>::new_from_slice(secret.0.expose_secret().as_bytes()).unwrap();
        mac.update(query_string.as_bytes());
        mac.verify_slice(&tag)?;

        Ok(self.error)
    }
}

pub async fn login_form(flash_message: IncomingFlashMessages) -> HttpResponse {
    let mut error_html = String::new();

    for m in flash_message.iter().filter(|m| m.level() == Level::Error) {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta http-equiv="content-type" content="text/html; charset=utf-8 IE=edge">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Login</title>
            </head>
            <body>
                {error_html}
                <form action="/login" method="post">
                    <label>
                        Username
                        <input type="text" name="username" placeholder="Enter username">
                    </label>
                    <label>
                        Password
                        <input type="password" name="password" placeholder="Enter password">
                    </label>
                    <button type="submit">Login</button>
                </form>
            </body>
            </html>
            "#
        ))
}
