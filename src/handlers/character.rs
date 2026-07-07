use actix_web::{web, HttpResponse, Result};
use serde_json::json;

pub async fn get_characters() -> Result<HttpResponse> {
    // TODO: Get all available characters
    Ok(HttpResponse::Ok().json(json!({
        "characters": [
            {
                "id": "char_1",
                "name": "Aleksandr",
                "specialty": "Russian",
                "avatar": "url_to_avatar"
            },
            {
                "id": "char_2", 
                "name": "Ahmet",
                "specialty": "Turkish",
                "avatar": "url_to_avatar"
            }
        ]
    })))
}

pub async fn get_character(char_id: web::Path<String>) -> Result<HttpResponse> {
    // TODO: Get specific character details
    Ok(HttpResponse::Ok().json(json!({
        "id": char_id.into_inner(),
        "name": "Character Name",
        "personality": "friendly",
        "messages": [
            "Hello! Ready to learn?",
            "Great job!"
        ]
    })))
}
