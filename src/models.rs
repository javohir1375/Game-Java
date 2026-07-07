use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub level: i32,
    pub experience: i32,
    pub coins: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub user_id: String,
    pub lessons_completed: i32,
    pub exercises_solved: i32,
    pub current_streak: i32,
    pub total_xp: i32,
    pub languages: Vec<LanguageProgress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageProgress {
    pub language: String,
    pub level: i32,
    pub progress: f32,
    pub words_learned: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub description: String,
    pub avatar_url: String,
    pub personality: String,
    pub specialty_language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub description: String,
    pub language: String,
    pub level: i32,
    pub content: String,
    pub words: Vec<WordItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordItem {
    pub english: String,
    pub translation: String,
    pub pronunciation: String,
    pub example: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub id: String,
    pub lesson_id: String,
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProgress {
    pub user_id: String,
    pub lesson_id: String,
    pub completed: bool,
    pub score: i32,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub rank: i32,
    pub username: String,
    pub level: i32,
    pub total_xp: i32,
    pub country: Option<String>,
}

// Request/Response DTOs
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct SubmitAnswerRequest {
    pub exercise_id: String,
    pub answer: String,
}

#[derive(Debug, Serialize)]
pub struct SubmitAnswerResponse {
    pub correct: bool,
    pub explanation: String,
    pub xp_earned: i32,
}
