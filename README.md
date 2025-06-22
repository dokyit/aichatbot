<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# AI Chat - Next-Gen AI Chatbot

A modern, full-stack AI chatbot built with Rust and Leptos, featuring multi-provider AI support, persistent memory, file uploads, and a beautiful floating chat interface.

## Features

### 🤖 Multi-Provider AI Support
- **Ollama** (local models) - Run AI models locally for privacy
- **OpenAI** - GPT-4, GPT-3.5-turbo
- **Anthropic** - Claude 3 models
- **Google Gemini** - Gemini Pro and Pro Vision
- **OpenRouter** - Access to 100+ models from various providers

### 💬 Modern Chat Interface
- **Floating chatbox** like Perplexity
- **T3 Chat-style** AI suggested questions
- **Three-dot thinking animation** with reasoning dropdown
- **Markdown rendering** with syntax highlighting
- **Code blocks** with copy buttons
- **LaTeX support** for mathematical expressions

### 🧠 Persistent Memory
- **Cross-chat memory** - AI remembers your preferences across all conversations
- **User context** - Stores name, preferences, and important information
- **Smart suggestions** - AI generates contextual follow-up questions

### 📁 File & Voice Support
- **Image uploads** - AI can see and analyze images
- **PDF processing** - Extract and understand PDF content
- **Voice input** - Speech-to-text functionality
- **Multiple file types** - Support for various document formats

### 🎨 Beautiful UI/UX
- **Responsive design** - Works on desktop and mobile
- **Dark/light mode** ready
- **Smooth animations** and transitions
- **Modern Tailwind CSS** styling

## Tech Stack

- **Frontend & Backend**: Leptos (Rust + WASM + SSR)
- **AI Integration**: rust-genai (multi-provider LLM client)
- **Database**: SQLite with SQLx
- **Styling**: Tailwind CSS
- **File Processing**: image, lopdf, whisper-rs
- **Markdown**: pulldown-cmark with syntax highlighting

## Quick Start

### Prerequisites

1. **Rust** (latest stable)
2. **Node.js** (for development tools)
3. **Ollama** (optional, for local models)

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd aibot
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env with your API keys
   ```

3. **Install dependencies**
   ```bash
   cargo build
   ```

4. **Run the development server**
   ```bash
   cargo leptos watch
   ```

5. **Open your browser**
   Navigate to `http://localhost:3000`

### Environment Variables

Create a `.env` file in the project root:

```env
# Database
DATABASE_URL=sqlite:./aibot.db

# AI Provider API Keys (optional)
OPENAI_API_KEY=your_openai_api_key
ANTHROPIC_API_KEY=your_anthropic_api_key
GEMINI_API_KEY=your_gemini_api_key
OPENROUTER_API_KEY=your_openrouter_api_key

# Ollama Configuration
OLLAMA_BASE_URL=http://localhost:11434

# Default Settings
DEFAULT_AI_PROVIDER=ollama
DEFAULT_MODEL=llama3.2
```

## Usage

### Starting with Ollama (Local Models)

1. **Install Ollama**
   ```bash
   # macOS/Linux
   curl -fsSL https://ollama.ai/install.sh | sh
   
   # Windows
   # Download from https://ollama.ai/download
   ```

2. **Pull a model**
   ```bash
   ollama pull llama3.2
   ```

3. **Start Ollama**
   ```bash
   ollama serve
   ```

4. **Run the chatbot**
   ```bash
   cargo leptos watch
   ```

### Using Cloud AI Providers

1. **Get API keys** from your preferred providers
2. **Add them to your `.env` file**
3. **Select the provider** in the model switcher dropdown
4. **Start chatting!**

## Development

### Project Structure

```
src/
├── app.rs              # Main application component
├── main.rs             # Server entry point
├── lib.rs              # Library exports
├── models.rs           # Data structures
├── database.rs         # Database operations
├── ai_service.rs       # AI provider integration
├── api.rs              # Server functions
└── components/         # UI components
    ├── chat_box.rs     # Main chat interface
    ├── message.rs      # Message display
    ├── model_switcher.rs # AI provider/model selection
    ├── file_upload.rs  # File upload handling
    ├── voice_input.rs  # Voice input component
    ├── thinking_animation.rs # Loading animation
    └── suggested_questions.rs # AI suggested questions
```

### Adding New AI Providers

1. **Update the `AIProvider` enum** in `models.rs`
2. **Add provider configuration** in `ai_service.rs`
3. **Implement client creation** in `AIService::new()`
4. **Add model list** in `get_available_models()`

### Database Migrations

The database is automatically initialized with the required tables. To add new migrations:

1. **Create a new SQL file** in `migrations/`
2. **Update the migration logic** in `database.rs`

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Leptos](https://leptos.dev/) - Full-stack Rust web framework
- [rust-genai](https://github.com/rust-genai/rust-genai) - Multi-provider LLM client
- [Ollama](https://ollama.ai/) - Local LLM runner
- [Tailwind CSS](https://tailwindcss.com/) - Utility-first CSS framework

## Roadmap

- [ ] User authentication and multi-user support
- [ ] Chat history export/import
- [ ] Advanced file processing (Excel, Word docs)
- [ ] Real-time collaboration
- [ ] Mobile app (React Native/Flutter)
- [ ] Plugin system for custom integrations
- [ ] Advanced memory management
- [ ] Voice output (text-to-speech)
- [ ] Image generation capabilities
- [ ] API for third-party integrations
