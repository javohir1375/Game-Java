use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use crate::models::SubmitAnswerRequest;

pub async fn get_exercises() -> Result<HttpResponse> {
    // TODO: Get all exercises
    Ok(HttpResponse::Ok().json(json!({
        "exercises": []
    })))
}

pub async fn submit_answer(body: web::Json<SubmitAnswerRequest>) -> Result<HttpResponse> {
    // TODO: Check answer and return result
    let is_correct = true; // Placeholder
    
    Ok(HttpResponse::Ok().json(json!({
        "correct": is_correct,
        "explanation": "Great job!",
        "xp_earned": if is_correct { 10 } else { 0 }
    })))
}
