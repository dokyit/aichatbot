use sqlx::{sqlite::SqlitePool, Row};
use anyhow::Result;
use crate::models::*;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        Self::run_migrations(&pool).await?;
        Ok(Self { pool })
    }

    async fn run_migrations(pool: &SqlitePool) -> Result<()> {
        let migration_sql = include_str!("../migrations/001_create_tables.sql");
        sqlx::query(migration_sql).execute(pool).await?;
        Ok(())
    }

    // User operations
    pub async fn create_user(&self, user: &User) -> Result<()> {
        sqlx::query!(
            "INSERT INTO users (id, name, email, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
            user.id,
            user.name,
            user.email,
            user.created_at,
            user.updated_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_user(&self, user_id: &str) -> Result<Option<User>> {
        let row = sqlx::query!(
            "SELECT id, name, email, created_at, updated_at FROM users WHERE id = ?",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.id,
            name: r.name,
            email: r.email,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    // Chat session operations
    pub async fn create_session(&self, session: &ChatSession) -> Result<()> {
        sqlx::query!(
            "INSERT INTO chat_sessions (id, user_id, title, model_provider, model_name, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            session.id,
            session.user_id,
            session.title,
            session.model_provider,
            session.model_name,
            session.created_at,
            session.updated_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_user_sessions(&self, user_id: &str) -> Result<Vec<ChatSession>> {
        let rows = sqlx::query!(
            "SELECT id, user_id, title, model_provider, model_name, created_at, updated_at FROM chat_sessions WHERE user_id = ? ORDER BY updated_at DESC",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| ChatSession {
                id: r.id,
                user_id: r.user_id,
                title: r.title,
                model_provider: r.model_provider,
                model_name: r.model_name,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }

    pub async fn get_session(&self, session_id: &str) -> Result<Option<ChatSession>> {
        let row = sqlx::query!(
            "SELECT id, user_id, title, model_provider, model_name, created_at, updated_at FROM chat_sessions WHERE id = ?",
            session_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| ChatSession {
            id: r.id,
            user_id: r.user_id,
            title: r.title,
            model_provider: r.model_provider,
            model_name: r.model_name,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    // Message operations
    pub async fn create_message(&self, message: &Message) -> Result<()> {
        sqlx::query!(
            "INSERT INTO messages (id, session_id, role, content, reasoning, model_provider, model_name, tokens_used, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            message.id,
            message.session_id,
            message.role.to_string(),
            message.content,
            message.reasoning,
            message.model_provider,
            message.model_name,
            message.tokens_used,
            message.created_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_session_messages(&self, session_id: &str) -> Result<Vec<Message>> {
        let rows = sqlx::query!(
            "SELECT id, session_id, role, content, reasoning, model_provider, model_name, tokens_used, created_at FROM messages WHERE session_id = ? ORDER BY created_at ASC",
            session_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Message {
                id: r.id,
                session_id: r.session_id,
                role: MessageRole::from(r.role),
                content: r.content,
                reasoning: r.reasoning,
                model_provider: r.model_provider,
                model_name: r.model_name,
                tokens_used: r.tokens_used,
                created_at: r.created_at,
            })
            .collect())
    }

    // User memory operations
    pub async fn save_memory(&self, memory: &UserMemory) -> Result<()> {
        sqlx::query!(
            "INSERT OR REPLACE INTO user_memory (id, user_id, memory_key, memory_value, confidence, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            memory.id,
            memory.user_id,
            memory.memory_key,
            memory.memory_value,
            memory.confidence,
            memory.created_at,
            memory.updated_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_user_memory(&self, user_id: &str) -> Result<Vec<UserMemory>> {
        let rows = sqlx::query!(
            "SELECT id, user_id, memory_key, memory_value, confidence, created_at, updated_at FROM user_memory WHERE user_id = ? ORDER BY confidence DESC, updated_at DESC",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| UserMemory {
                id: r.id,
                user_id: r.user_id,
                memory_key: r.memory_key,
                memory_value: r.memory_value,
                confidence: r.confidence,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }

    pub async fn get_memory_by_key(&self, user_id: &str, memory_key: &str) -> Result<Option<UserMemory>> {
        let row = sqlx::query!(
            "SELECT id, user_id, memory_key, memory_value, confidence, created_at, updated_at FROM user_memory WHERE user_id = ? AND memory_key = ?",
            user_id,
            memory_key
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| UserMemory {
            id: r.id,
            user_id: r.user_id,
            memory_key: r.memory_key,
            memory_value: r.memory_value,
            confidence: r.confidence,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    // File attachment operations
    pub async fn save_file_attachment(&self, attachment: &FileAttachment) -> Result<()> {
        sqlx::query!(
            "INSERT INTO file_attachments (id, message_id, file_name, file_path, file_type, file_size, content_hash, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            attachment.id,
            attachment.message_id,
            attachment.file_name,
            attachment.file_path,
            attachment.file_type,
            attachment.file_size,
            attachment.content_hash,
            attachment.created_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_message_attachments(&self, message_id: &str) -> Result<Vec<FileAttachment>> {
        let rows = sqlx::query!(
            "SELECT id, message_id, file_name, file_path, file_type, file_size, content_hash, created_at FROM file_attachments WHERE message_id = ?",
            message_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| FileAttachment {
                id: r.id,
                message_id: r.message_id,
                file_name: r.file_name,
                file_path: r.file_path,
                file_type: r.file_type,
                file_size: r.file_size,
                content_hash: r.content_hash,
                created_at: r.created_at,
            })
            .collect())
    }

    // Suggested questions operations
    pub async fn save_suggested_questions(&self, questions: &[SuggestedQuestion]) -> Result<()> {
        for question in questions {
            sqlx::query!(
                "INSERT INTO suggested_questions (id, session_id, question, relevance_score, used, created_at) VALUES (?, ?, ?, ?, ?, ?)",
                question.id,
                question.session_id,
                question.question,
                question.relevance_score,
                question.used,
                question.created_at
            )
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    pub async fn get_session_suggested_questions(&self, session_id: &str, limit: i64) -> Result<Vec<SuggestedQuestion>> {
        let rows = sqlx::query!(
            "SELECT id, session_id, question, relevance_score, used, created_at FROM suggested_questions WHERE session_id = ? AND used = FALSE ORDER BY relevance_score DESC, created_at DESC LIMIT ?",
            session_id,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| SuggestedQuestion {
                id: r.id,
                session_id: r.session_id,
                question: r.question,
                relevance_score: r.relevance_score,
                used: r.used,
                created_at: r.created_at,
            })
            .collect())
    }

    pub async fn mark_question_used(&self, question_id: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE suggested_questions SET used = TRUE WHERE id = ?",
            question_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
} 