use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub user_id: String,
    pub title: Option<String>,
    pub model_provider: String,
    pub model_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub session_id: String,
    pub role: MessageRole,
    pub content: String,
    pub reasoning: Option<String>,
    pub model_provider: Option<String>,
    pub model_name: Option<String>,
    pub tokens_used: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

impl std::fmt::Display for MessageRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageRole::User => write!(f, "user"),
            MessageRole::Assistant => write!(f, "assistant"),
            MessageRole::System => write!(f, "system"),
        }
    }
}

impl From<String> for MessageRole {
    fn from(s: String) -> Self {
        match s.as_str() {
            "user" => MessageRole::User,
            "assistant" => MessageRole::Assistant,
            "system" => MessageRole::System,
            _ => MessageRole::User,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMemory {
    pub id: String,
    pub user_id: String,
    pub memory_key: String,
    pub memory_value: String,
    pub confidence: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAttachment {
    pub id: String,
    pub message_id: String,
    pub file_name: String,
    pub file_path: String,
    pub file_type: String,
    pub file_size: i64,
    pub content_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedQuestion {
    pub id: String,
    pub session_id: String,
    pub question: String,
    pub relevance_score: f64,
    pub used: bool,
    pub created_at: DateTime<Utc>,
}

// AI Provider Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIProvider {
    Ollama,
    OpenAI,
    Anthropic,
    Gemini,
    OpenRouter,
}

impl std::fmt::Display for AIProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AIProvider::Ollama => write!(f, "ollama"),
            AIProvider::OpenAI => write!(f, "openai"),
            AIProvider::Anthropic => write!(f, "anthropic"),
            AIProvider::Gemini => write!(f, "gemini"),
            AIProvider::OpenRouter => write!(f, "openrouter"),
        }
    }
}

impl From<String> for AIProvider {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ollama" => AIProvider::Ollama,
            "openai" => AIProvider::OpenAI,
            "anthropic" => AIProvider::Anthropic,
            "gemini" => AIProvider::Gemini,
            "openrouter" => AIProvider::OpenRouter,
            _ => AIProvider::Ollama,
        }
    }
}

// Request/Response Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub session_id: Option<String>,
    pub message: String,
    pub model_provider: Option<AIProvider>,
    pub model_name: Option<String>,
    pub files: Vec<FileUpload>,
    pub voice_data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub message_id: String,
    pub content: String,
    pub reasoning: Option<String>,
    pub suggested_questions: Vec<String>,
    pub model_provider: String,
    pub model_name: String,
    pub tokens_used: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUpload {
    pub name: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub title: Option<String>,
    pub model_provider: AIProvider,
    pub model_name: String,
}

// Utility functions
impl User {
    pub fn new(name: Option<String>, email: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            email,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl ChatSession {
    pub fn new(user_id: String, model_provider: AIProvider, model_name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            title: None,
            model_provider: model_provider.to_string(),
            model_name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl Message {
    pub fn new(session_id: String, role: MessageRole, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            session_id,
            role,
            content,
            reasoning: None,
            model_provider: None,
            model_name: None,
            tokens_used: None,
            created_at: Utc::now(),
        }
    }
}

impl UserMemory {
    pub fn new(user_id: String, memory_key: String, memory_value: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            memory_key,
            memory_value,
            confidence: 1.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
} 