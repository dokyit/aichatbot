use leptos::*;
use leptos_router::*;
use crate::{
    models::*,
    api::*,
    components::{
        message::MessageComponent,
        suggested_questions::SuggestedQuestions,
        model_switcher::ModelSwitcher,
        file_upload::FileUpload,
        voice_input::VoiceInput,
        thinking_animation::ThinkingAnimation,
    },
};

#[component]
pub fn ChatBox() -> impl IntoView {
    let (current_session, set_current_session) = create_signal(None::<String>);
    let (messages, set_messages) = create_signal(Vec::<Message>::new());
    let (input_value, set_input_value) = create_signal(String::new());
    let (is_loading, set_is_loading) = create_signal(false);
    let (suggested_questions, set_suggested_questions) = create_signal(Vec::<SuggestedQuestion>::new());
    let (selected_model, set_selected_model) = create_signal(AIProvider::Ollama);
    let (selected_model_name, set_selected_model_name) = create_signal("llama3.2".to_string());
    let (uploaded_files, set_uploaded_files) = create_signal(Vec::<FileUpload>::new());

    // Create a new session when component mounts
    create_effect(move |_| {
        spawn_local(async move {
            match create_session(None, selected_model.get(), selected_model_name.get()).await {
                Ok(session_id) => {
                    set_current_session.set(Some(session_id));
                }
                Err(e) => {
                    log::error!("Failed to create session: {}", e);
                }
            }
        });
    });

    // Load messages when session changes
    create_effect(move |_| {
        if let Some(session_id) = current_session.get() {
            spawn_local(async move {
                match get_chat_history(session_id).await {
                    Ok(msgs) => set_messages.set(msgs),
                    Err(e) => log::error!("Failed to load messages: {}", e),
                }
            });
        }
    });

    // Load suggested questions
    create_effect(move |_| {
        if let Some(session_id) = current_session.get() {
            spawn_local(async move {
                match get_suggested_questions(session_id).await {
                    Ok(questions) => set_suggested_questions.set(questions),
                    Err(e) => log::error!("Failed to load suggested questions: {}", e),
                }
            });
        }
    });

    let send_message = create_action(|input: &(String, Vec<FileUpload>)| {
        let (message, files) = input.clone();
        async move {
            if let Some(session_id) = current_session.get() {
                set_is_loading.set(true);
                let result = send_message(session_id, message, files).await;
                set_is_loading.set(false);
                result
            } else {
                Err(anyhow::anyhow!("No active session"))
            }
        }
    });

    let handle_send = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let message = input_value.get();
        if !message.trim().is_empty() {
            let files = uploaded_files.get();
            send_message.dispatch((message, files));
            set_input_value.set(String::new());
            set_uploaded_files.set(Vec::new());
        }
    };

    let handle_suggested_question = move |question: String| {
        set_input_value.set(question);
    };

    let handle_file_upload = move |files: Vec<FileUpload>| {
        set_uploaded_files.set(files);
    };

    let handle_model_change = move |provider: AIProvider, model_name: String| {
        set_selected_model.set(provider);
        set_selected_model_name.set(model_name);
        // Create new session with new model
        spawn_local(async move {
            match create_session(None, provider, model_name).await {
                Ok(session_id) => {
                    set_current_session.set(Some(session_id));
                    set_messages.set(Vec::new());
                }
                Err(e) => {
                    log::error!("Failed to create session: {}", e);
                }
            }
        });
    };

    view! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 p-4">
            <div class="max-w-4xl mx-auto">
                // Header with model switcher
                <div class="bg-white rounded-lg shadow-lg p-4 mb-6">
                    <div class="flex items-center justify-between">
                        <h1 class="text-2xl font-bold text-gray-800">"AI Chat"</h1>
                        <ModelSwitcher
                            selected_provider=selected_model
                            selected_model=selected_model_name
                            on_change=handle_model_change
                        />
                    </div>
                </div>

                // Messages area
                <div class="bg-white rounded-lg shadow-lg p-6 mb-6 min-h-96 max-h-96 overflow-y-auto">
                    <div class="space-y-4">
                        {move || {
                            messages.get().into_iter().map(|msg| {
                                view! {
                                    <MessageComponent message=msg />
                                }
                            }).collect::<Vec<_>>()
                        }}
                        {move || {
                            if is_loading.get() {
                                view! {
                                    <ThinkingAnimation />
                                }
                            } else {
                                view! { <div></div> }
                            }
                        }}
                    </div>
                </div>

                // Suggested questions
                {move || {
                    let questions = suggested_questions.get();
                    if !questions.is_empty() {
                        view! {
                            <SuggestedQuestions
                                questions=questions
                                on_question_click=handle_suggested_question
                            />
                        }
                    } else {
                        view! { <div></div> }
                    }
                }}

                // Floating input box
                <div class="fixed bottom-6 left-1/2 transform -translate-x-1/2 w-full max-w-2xl">
                    <div class="bg-white rounded-full shadow-2xl border border-gray-200">
                        <form on:submit=handle_send class="flex items-center p-2">
                            // File upload button
                            <FileUpload on_upload=handle_file_upload />
                            
                            // Voice input button
                            <VoiceInput />
                            
                            // Text input
                            <input
                                type="text"
                                placeholder="Ask me anything..."
                                class="flex-1 px-4 py-2 text-gray-700 bg-transparent border-none outline-none"
                                prop:value=move || input_value.get()
                                on:input=move |ev| {
                                    set_input_value.set(event_target_value(&ev));
                                }
                            />
                            
                            // Send button
                            <button
                                type="submit"
                                disabled=move || is_loading.get() || input_value.get().trim().is_empty()
                                class="ml-2 p-2 bg-blue-600 text-white rounded-full hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                            >
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path>
                                </svg>
                            </button>
                        </form>
                    </div>
                    
                    // File preview
                    {move || {
                        let files = uploaded_files.get();
                        if !files.is_empty() {
                            view! {
                                <div class="mt-2 bg-white rounded-lg shadow-lg p-3">
                                    <div class="text-sm text-gray-600 mb-2">"Attached files:"</div>
                                    <div class="space-y-1">
                                        {files.into_iter().map(|file| {
                                            view! {
                                                <div class="flex items-center text-sm">
                                                    <span class="text-gray-800">{file.name}</span>
                                                    <span class="text-gray-500 ml-2">"({file.data.len()} bytes)"</span>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }
                        } else {
                            view! { <div></div> }
                        }
                    }}
                </div>
            </div>
        </div>
    }
} 