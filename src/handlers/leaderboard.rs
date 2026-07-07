use actix_web::{web, HttpResponse, Result};
use serde_json::json;

pub async fn get_leaderboard() -> Result<HttpResponse> {
    // TODO: Get global leaderboard
    Ok(HttpResponse::Ok().json(json!({
        "leaderboard": []
    })))
}

pub async fn get_by_language(language: web::Path<String>) -> Result<HttpResponse> {
    // TODO: Get leaderboard by language
    Ok(HttpResponse::Ok().json(json!({
        "language": language.into_inner(),
        "leaderboard": []
    })))
}
