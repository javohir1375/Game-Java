mod handlers;
mod models;
mod db;
mod config;
mod utils;

use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    println!("🚀 Language Learning Game Server Starting...");
    
    // Initialize database
    let db_pool = db::init_db().await
        .expect("Failed to initialize database");
    
    let pool = Arc::new(db_pool);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            // Auth routes
            .route("/api/auth/register", web::post().to(handlers::auth::register))
            .route("/api/auth/login", web::post().to(handlers::auth::login))
            .route("/api/auth/me", web::get().to(handlers::auth::get_me))
            // User routes
            .route("/api/users/{id}/profile", web::get().to(handlers::user::get_profile))
            .route("/api/users/{id}/stats", web::get().to(handlers::user::get_stats))
            .route("/api/users/{id}/update", web::put().to(handlers::user::update_profile))
            // Character routes
            .route("/api/characters", web::get().to(handlers::character::get_characters))
            .route("/api/characters/{id}", web::get().to(handlers::character::get_character))
            // Lesson routes
            .route("/api/lessons", web::get().to(handlers::lesson::get_lessons))
            .route("/api/lessons/{id}", web::get().to(handlers::lesson::get_lesson))
            .route("/api/lessons/{id}/complete", web::post().to(handlers::lesson::complete_lesson))
            // Exercise routes
            .route("/api/exercises", web::get().to(handlers::exercise::get_exercises))
            .route("/api/exercises/{id}/submit", web::post().to(handlers::exercise::submit_answer))
            // Leaderboard routes
            .route("/api/leaderboard", web::get().to(handlers::leaderboard::get_leaderboard))
            .route("/api/leaderboard/{language}", web::get().to(handlers::leaderboard::get_by_language))
            // Health check
            .route("/health", web::get().to(handlers::health::check))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
