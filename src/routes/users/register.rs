use actix_web::{post, HttpResponse, Responder, web};
use mongodb::{Database, bson::doc};
use serde::Deserialize;
use serde_json::json;
use regex::Regex;
use bcrypt::{hash, DEFAULT_COST};
use crate::models::user_model::User;
use crate::controllers::auth::{generate_jwt};

#[derive(Debug, Deserialize)]
pub struct Body {
    username: String,
    password: String,
}

#[post("/users/register")]
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
            "error": "Password can't be empty",
            "error_code": "EMPTY_PASSWORD",
        }));
    }
    
    let user: Option<User> = db.collection::<User>("users").find_one(doc! {
        "username": body.username.to_owned(),
    }, None).await.unwrap();

    if !user.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "error": "Username already taken",
            "error_code": "USERNAME_TAKEN",
        }));
    }

    let password_hash: String = hash(body.password.to_owned(), DEFAULT_COST).unwrap();
    let token: String = generate_jwt(body.username.to_owned());

    db.collection::<User>("users").insert_one(User {
        username: body.username.to_owned(),
        password: password_hash,
        token: token,
    }, None).await.unwrap();
    
    HttpResponse::Ok().json(json!({
        "success": true,
    }))
}