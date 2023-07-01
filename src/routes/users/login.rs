use actix_web::{post, HttpResponse, Responder, web};
use crate::models::user_model::User;
use mongodb::{Database, bson::doc};
use serde::Deserialize;
use serde_json::json;
use regex::Regex;

#[derive(Debug, Deserialize)]
pub struct Body {
    username: String,
    password: String,
}

#[post("/users/login")]
pub async fn handler(body: web::Json<Body>, db: web::Data<Database>) -> impl Responder {
    let username_check: Regex = Regex::new(r"^[a-zA-Z0-9_.]*$").unwrap();
    if !username_check.is_match(body.username.as_str()) {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "error": "Username can contain only english alphanumeric characters, underscore and period",
            "error_code": "BAD_USERNAME_CHARS",
        }));
    }

    if body.username.trim().len() < 3 || body.username.trim().len() > 15 {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "error": "Username length must be >3 and <15",
            "error_code": "BAD_USERNAME_LENGTH",
        }));
    }
    
    if body.password.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "error": "Invalid username or password",
            "error_code": "INVALID_CREDENTIALS",
        }));
    }
    
    let user: Option<User> = db.collection::<User>("users").find_one(doc! {
        "username": body.username.to_owned(),
    }, None).await.unwrap();

    if user.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "error": "Invalid username or password",
            "error_code": "INVALID_CREDENTIALS",
        }));
    }
    
    HttpResponse::Ok().json(json!({
        "success": true,
    }))
}