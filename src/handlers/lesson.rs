use actix_web::{web, HttpResponse, Result};
use serde_json::json;

pub async fn get_lessons() -> Result<HttpResponse> {
    // TODO: Get all lessons
    Ok(HttpResponse::Ok().json(json!({
        "lessons": []
    })))
}

pub async fn get_lesson(lesson_id: web::Path<String>) -> Result<HttpResponse> {
    // TODO: Get specific lesson
    Ok(HttpResponse::Ok().json(json!({
        "id": lesson_id.into_inner(),
        "title": "Lesson Title",
        "content": "Lesson content"
    })))
}

pub async fn complete_lesson(lesson_id: web::Path<String>, body: web::Json<serde_json::Value>) -> Result<HttpResponse> {
    // TODO: Mark lesson as complete
    Ok(HttpResponse::Ok().json(json!({
        "message": "Lesson completed!",
        "xp_earned": 50
    })))
}
