use leptos::*;
use anyhow::Result;
use crate::{
    models::*,
    database::Database,
    ai_service::{AIService, AIServiceConfig},
};

// Server state
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub ai_service: AIService,
}

// Server function to create a new chat session
#[server(CreateSession, "/api")]
pub async fn create_session(
    title: Option<String>,
    model_provider: AIProvider,
    model_name: String,
) -> Result<String> {
    let state = use_context::<AppState>()
        .ok_or_else(|| anyhow::anyhow!("AppState not found"))?;
    
    // For now, we'll use a default user ID
    // In a real app, you'd get this from authentication
    let user_id = "default_user".to_string();
    
    let session = ChatSession::new(user_id, model_provider, model_name);
    session.title = title;
    
    state.db.create_session(&session).await?;
    
    Ok(session.id)
}

// Server function to send a chat message
#[server(SendMessage, "/api")]
pub async fn send_message(
    session_id: String,
    message: String,
    files: Vec<FileUpload>,
) -> Result<ChatResponse> {
    let state = use_context::<AppState>()
        .ok_or_else(|| anyhow::anyhow!("AppState not found"))?;
    
    // Get the session
    let session = state.db.get_session(&session_id).await?
        .ok_or_else(|| anyhow::anyhow!("Session not found"))?;
    
    // Get user memory
    let user_memory = state.db.get_user_memory(&session.user_id).await?;
    
    // Get session messages
    let messages = state.db.get_session_messages(&session_id).await?;
    
    // Create user message
    let user_message = Message::new(session_id.clone(), MessageRole::User, message.clone());
    state.db.create_message(&user_message).await?;
    
    // Save file attachments if any
    for file in &files {
        let attachment = FileAttachment {
            id: uuid::Uuid::new_v4().to_string(),
            message_id: user_message.id.clone(),
            file_name: file.name.clone(),
            file_path: format!("uploads/{}", file.name),
            file_type: file.content_type.clone(),
            file_size: file.data.len() as i64,
            content_hash: None,
            created_at: chrono::Utc::now(),
        };
        state.db.save_file_attachment(&attachment).await?;
    }
    
    // Get AI provider and model
    let provider = AIProvider::from(session.model_provider.clone());
    let model_name = session.model_name.clone();
    
    // Send to AI service
    let ai_response = state.ai_service.chat(
        provider,
        &model_name,
        messages,
        &user_memory,
        &files,
    ).await?;
    
    // Save AI response
    let ai_message = Message {
        id: ai_response.message_id.clone(),
        session_id: session_id.clone(),
        role: MessageRole::Assistant,
        content: ai_response.content.clone(),
        reasoning: ai_response.reasoning.clone(),
        model_provider: Some(ai_response.model_provider.clone()),
        model_name: Some(ai_response.model_name.clone()),
        tokens_used: ai_response.tokens_used,
        created_at: chrono::Utc::now(),
    };
    state.db.create_message(&ai_message).await?;
    
    // Save suggested questions
    let suggested_questions: Vec<SuggestedQuestion> = ai_response.suggested_questions
        .iter()
        .enumerate()
        .map(|(i, question)| SuggestedQuestion {
            id: uuid::Uuid::new_v4().to_string(),
            session_id: session_id.clone(),
            question: question.clone(),
            relevance_score: 1.0 - (i as f64 * 0.1),
            used: false,
            created_at: chrono::Utc::now(),
        })
        .collect();
    
    if !suggested_questions.is_empty() {
        state.db.save_suggested_questions(&suggested_questions).await?;
    }
    
    Ok(ai_response)
}

// Server function to get chat history
#[server(GetChatHistory, "/api")]
pub async fn get_chat_history(session_id: String) -> Result<Vec<Message>> {
    let state = use_context::<AppState>()
        .ok_or_else(|| anyhow::anyhow!("AppState not found"))?;
    
    state.db.get_session_messages(&session_id).await
}

// Server function to get user sessions
#[server(GetUserSessions, "/api")]
pub async fn get_user_sessions() -> Result<Vec<ChatSession>> {
    let state = use_context::<AppState>()
        .ok_or_else(|| anyhow::anyhow!("AppState not found"))?;
    
    // For now, use default user
    let user_id = "default_user".to_string();
    state.db.get_user_sessions(&user_id).await
}

// Server function to get suggested questions
#[server(GetSuggestedQuestions, "/api")]
pub async fn get_suggested_questions(session_id: String) -> Result<Vec<SuggestedQuestion>> {
    let state = use_context::<AppState>()
        .ok_or_else(|| anyhow::anyhow!("AppState not found"))?;
    
    state.db.get_session_suggested_questions(&session_id, 5).await
}

// Server function to save user memory
#[server(SaveMemory, "/api")]
pub async fn save_memory(memory_key: String, memory_value: String) -> Result<()> {
    let state = use_context::<AppState>()
        .ok_or_else(|| anyhow::anyhow!("AppState not found"))?;
    
    // For now, use default user
    let user_id = "default_user".to_string();
    
    let memory = UserMemory::new(user_id, memory_key, memory_value);
    state.db.save_memory(&memory).await
}

// Server function to get user memory
#[server(GetUserMemory, "/api")]
pub async fn get_user_memory() -> Result<Vec<UserMemory>> {
    let state = use_context::<AppState>()
        .ok_or_else(|| anyhow::anyhow!("AppState not found"))?;
    
    // For now, use default user
    let user_id = "default_user".to_string();
    state.db.get_user_memory(&user_id).await
}

// Server function to get available models
#[server(GetAvailableModels, "/api")]
pub async fn get_available_models(provider: AIProvider) -> Result<Vec<String>> {
    let state = use_context::<AppState>()
        .ok_or_else(|| anyhow::anyhow!("AppState not found"))?;
    
    state.ai_service.get_available_models(provider).await
}

// Server function to handle voice input
#[server(ProcessVoiceInput, "/api")]
pub async fn process_voice_input(audio_data: Vec<u8>) -> Result<String> {
    // This would use whisper-rs to transcribe audio
    // For now, return a placeholder
    Ok("Voice input processed".to_string())
}

// Server function to mark suggested question as used
#[server(MarkQuestionUsed, "/api")]
pub async fn mark_question_used(question_id: String) -> Result<()> {
    let state = use_context::<AppState>()
        .ok_or_else(|| anyhow::anyhow!("AppState not found"))?;
    
    state.db.mark_question_used(&question_id).await
} 