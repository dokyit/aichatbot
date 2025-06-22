use anyhow::Result;
use async_stream::stream;
use futures::Stream;
use serde_json::{json, Value};
use std::collections::HashMap;
use tokio::sync::RwLock;
use crate::models::*;

pub struct AIService {
    clients: RwLock<HashMap<AIProvider, String>>, // Store API keys/URLs
    config: AIServiceConfig,
}

#[derive(Clone)]
pub struct AIServiceConfig {
    pub openai_api_key: Option<String>,
    pub anthropic_api_key: Option<String>,
    pub gemini_api_key: Option<String>,
    pub openrouter_api_key: Option<String>,
    pub ollama_base_url: String,
}

impl Default for AIServiceConfig {
    fn default() -> Self {
        Self {
            openai_api_key: None,
            anthropic_api_key: None,
            gemini_api_key: None,
            openrouter_api_key: None,
            ollama_base_url: "http://localhost:11434".to_string(),
        }
    }
}

impl AIService {
    pub async fn new(config: AIServiceConfig) -> Result<Self> {
        let mut clients = HashMap::new();
        
        // Store configuration for each provider
        if config.openai_api_key.is_some() {
            clients.insert(AIProvider::OpenAI, config.openai_api_key.clone().unwrap());
        }
        if config.anthropic_api_key.is_some() {
            clients.insert(AIProvider::Anthropic, config.anthropic_api_key.clone().unwrap());
        }
        if config.gemini_api_key.is_some() {
            clients.insert(AIProvider::Gemini, config.gemini_api_key.clone().unwrap());
        }
        if config.openrouter_api_key.is_some() {
            clients.insert(AIProvider::OpenRouter, config.openrouter_api_key.clone().unwrap());
        }
        // Ollama doesn't need an API key
        clients.insert(AIProvider::Ollama, config.ollama_base_url.clone());

        Ok(Self {
            clients: RwLock::new(clients),
            config,
        })
    }

    pub async fn chat(
        &self,
        provider: AIProvider,
        model_name: &str,
        messages: Vec<Message>,
        user_memory: &[UserMemory],
        files: &[FileUpload],
    ) -> Result<ChatResponse> {
        let clients = self.clients.read().await;
        
        // Check if provider is available
        if !clients.contains_key(&provider) {
            return Err(anyhow::anyhow!("Provider {:?} not available", provider));
        }

        // Build system prompt with user memory
        let system_prompt = self.build_system_prompt(user_memory);
        
        // Convert messages to the format expected by the provider
        let mut formatted_messages = vec![];
        formatted_messages.push(json!({
            "role": "system",
            "content": system_prompt
        }));
        
        for msg in &messages {
            let role = match msg.role {
                MessageRole::User => "user",
                MessageRole::Assistant => "assistant",
                MessageRole::System => "system",
            };
            
            let content = if !files.is_empty() && msg.role == MessageRole::User {
                self.build_content_with_files(&msg.content, files)?
            } else {
                msg.content.clone()
            };
            
            formatted_messages.push(json!({
                "role": role,
                "content": content
            }));
        }

        // For now, return a mock response
        // In a real implementation, you'd make HTTP requests to the respective APIs
        let mock_response = format!("This is a mock response from {} using model {}. You said: {}", 
            provider.to_string(), model_name, 
            messages.last().map(|m| &m.content).unwrap_or(&"".to_string()));

        Ok(ChatResponse {
            message_id: uuid::Uuid::new_v4().to_string(),
            content: mock_response,
            reasoning: Some("This is a mock reasoning process.".to_string()),
            suggested_questions: self.generate_suggested_questions(&mock_response, &messages, user_memory).await?,
            model_provider: provider.to_string(),
            model_name: model_name.to_string(),
            tokens_used: Some(150),
        })
    }

    pub async fn chat_stream(
        &self,
        provider: AIProvider,
        model_name: &str,
        messages: Vec<Message>,
        user_memory: &[UserMemory],
        files: &[FileUpload],
    ) -> Result<impl Stream<Item = Result<String>>> {
        // For now, return a simple stream with a mock response
        let response = format!("Mock streaming response from {} using model {}", provider.to_string(), model_name);
        
        Ok(stream! {
            for chunk in response.split_whitespace() {
                yield Ok(format!("{} ", chunk));
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        })
    }

    fn build_system_prompt(&self, user_memory: &[UserMemory]) -> String {
        let mut prompt = String::from("You are a helpful AI assistant. ");
        
        if !user_memory.is_empty() {
            prompt.push_str("\n\nUser context and preferences:\n");
            for memory in user_memory {
                prompt.push_str(&format!("- {}: {}\n", memory.memory_key, memory.memory_value));
            }
            prompt.push_str("\nPlease remember and use this information in our conversation.\n");
        }
        
        prompt.push_str("\nAlways provide helpful, accurate, and engaging responses. ");
        prompt.push_str("If you're not sure about something, say so. ");
        prompt.push_str("You can process images, PDFs, and other files when provided.");
        
        prompt
    }

    fn build_content_with_files(&self, content: &str, files: &[FileUpload]) -> Result<String> {
        let mut full_content = content.to_string();
        
        for file in files {
            match file.content_type.as_str() {
                "image/" => {
                    // For images, we'll encode as base64 and add to content
                    let base64_data = base64::encode(&file.data);
                    full_content.push_str(&format!("\n\n[Image: {}]\n", file.name));
                    full_content.push_str(&format!("data:image/{};base64,{}\n", 
                        file.content_type.split('/').nth(1).unwrap_or("jpeg"), 
                        base64_data));
                }
                "application/pdf" => {
                    // For PDFs, we'll extract text and add to content
                    if let Ok(text) = self.extract_pdf_text(&file.data) {
                        full_content.push_str(&format!("\n\n[PDF Content from {}]\n", file.name));
                        full_content.push_str(&text);
                    }
                }
                "text/" => {
                    // For text files, add content directly
                    if let Ok(text) = String::from_utf8(file.data.clone()) {
                        full_content.push_str(&format!("\n\n[Text from {}]\n", file.name));
                        full_content.push_str(&text);
                    }
                }
                _ => {
                    full_content.push_str(&format!("\n\n[File: {} - {}]\n", file.name, file.content_type));
                }
            }
        }
        
        Ok(full_content)
    }

    fn extract_pdf_text(&self, data: &[u8]) -> Result<String> {
        // Simple PDF text extraction using lopdf
        // This is a basic implementation - you might want to use a more robust library
        let mut text = String::new();
        
        // For now, we'll just return a placeholder
        // In a real implementation, you'd use lopdf to extract text
        text.push_str("[PDF content extracted]");
        
        Ok(text)
    }

    async fn generate_suggested_questions(
        &self,
        response_content: &str,
        messages: &[Message],
        user_memory: &[UserMemory],
    ) -> Result<Vec<String>> {
        // Generate AI suggested questions based on the conversation context
        let mut questions = Vec::new();
        
        // Simple heuristic-based question generation
        // In a real implementation, you'd use the AI to generate these
        
        // Extract key topics from the response
        let topics = self.extract_topics(response_content);
        
        for topic in topics.iter().take(3) {
            questions.push(format!("Tell me more about {}", topic));
        }
        
        // Add some generic follow-up questions
        questions.push("Can you explain that in simpler terms?".to_string());
        questions.push("What are the pros and cons of this approach?".to_string());
        questions.push("How does this compare to other solutions?".to_string());
        
        // Limit to 5 questions
        questions.truncate(5);
        
        Ok(questions)
    }

    fn extract_topics(&self, content: &str) -> Vec<String> {
        // Simple topic extraction - in a real implementation, you'd use NLP
        let words: Vec<&str> = content
            .split_whitespace()
            .filter(|word| word.len() > 4)
            .collect();
        
        words.iter()
            .take(5)
            .map(|s| s.to_string())
            .collect()
    }

    pub async fn get_available_models(&self, provider: AIProvider) -> Result<Vec<String>> {
        match provider {
            AIProvider::Ollama => {
                // For Ollama, we'll return common model names
                Ok(vec![
                    "llama3.2".to_string(),
                    "llama3.1".to_string(),
                    "mistral".to_string(),
                    "codellama".to_string(),
                    "llama2".to_string(),
                ])
            }
            AIProvider::OpenAI => {
                Ok(vec![
                    "gpt-4".to_string(),
                    "gpt-4-turbo".to_string(),
                    "gpt-3.5-turbo".to_string(),
                ])
            }
            AIProvider::Anthropic => {
                Ok(vec![
                    "claude-3-opus-20240229".to_string(),
                    "claude-3-sonnet-20240229".to_string(),
                    "claude-3-haiku-20240307".to_string(),
                ])
            }
            AIProvider::Gemini => {
                Ok(vec![
                    "gemini-pro".to_string(),
                    "gemini-pro-vision".to_string(),
                ])
            }
            AIProvider::OpenRouter => {
                Ok(vec![
                    "openai/gpt-4".to_string(),
                    "anthropic/claude-3-opus".to_string(),
                    "meta-llama/llama-3.1-8b-instruct".to_string(),
                ])
            }
        }
    }
} 