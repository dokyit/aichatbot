use leptos::*;
use crate::models::*;

#[component]
pub fn MessageComponent(message: Message) -> impl IntoView {
    let (show_reasoning, set_show_reasoning) = create_signal(false);

    let is_user = move || matches!(message.role, MessageRole::User);
    let is_assistant = move || matches!(message.role, MessageRole::Assistant);

    let toggle_reasoning = move |_| {
        set_show_reasoning.update(|show| *show = !*show);
    };

    let copy_to_clipboard = move |text: String| {
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Some(navigator) = window.navigator().clipboard() {
                    let _ = navigator.write_text(&text).await;
                }
            }
        });
    };

    view! {
        <div class=move || {
            if is_user() {
                "flex justify-end"
            } else {
                "flex justify-start"
            }
        }>
            <div class=move || {
                let base_classes = "max-w-3xl rounded-lg p-4";
                if is_user() {
                    format!("{} bg-blue-600 text-white", base_classes)
                } else {
                    format!("{} bg-gray-100 text-gray-800", base_classes)
                }
            }>
                // Message content with markdown rendering
                <div class="prose prose-sm max-w-none">
                    {move || render_markdown(&message.content)}
                </div>
                
                // Reasoning dropdown (only for assistant messages)
                {move || {
                    if is_assistant() && message.reasoning.is_some() {
                        view! {
                            <div class="mt-3 pt-3 border-t border-gray-200">
                                <button
                                    on:click=toggle_reasoning
                                    class="text-sm text-gray-500 hover:text-gray-700 flex items-center"
                                >
                                    <span>"Reasoning"</span>
                                    <svg
                                        class=move || {
                                            if show_reasoning.get() {
                                                "w-4 h-4 ml-1 transform rotate-180"
                                            } else {
                                                "w-4 h-4 ml-1"
                                            }
                                        }
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                                    </svg>
                                </button>
                                
                                {move || {
                                    if show_reasoning.get() {
                                        view! {
                                            <div class="mt-2 p-3 bg-gray-50 rounded text-sm">
                                                {message.reasoning.as_ref().unwrap()}
                                            </div>
                                        }
                                    } else {
                                        view! { <div></div> }
                                    }
                                }}
                            </div>
                        }
                    } else {
                        view! { <div></div> }
                    }
                }}
                
                // Message metadata
                <div class="mt-2 text-xs text-gray-500 flex items-center justify-between">
                    <span>{format!("{}", message.created_at.format("%H:%M"))}</span>
                    {move || {
                        if let Some(tokens) = message.tokens_used {
                            view! {
                                <span>"{} tokens"</span>
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

fn render_markdown(content: &str) -> Vec<View> {
    use pulldown_cmark::{Parser, Event, Tag, CodeBlockKind};
    
    let parser = Parser::new(content);
    let mut elements = Vec::new();
    let mut current_text = String::new();
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_content = String::new();
    
    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                if !current_text.is_empty() {
                    elements.push(view! {
                        <p class="mb-2">{current_text.clone()}</p>
                    });
                    current_text.clear();
                }
                in_code_block = true;
                code_lang = lang.to_string();
            }
            Event::End(Tag::CodeBlock(_)) => {
                if in_code_block {
                    elements.push(view! {
                        <CodeBlock
                            language=code_lang.clone()
                            content=code_content.clone()
                        />
                    });
                    in_code_block = false;
                    code_content.clear();
                    code_lang.clear();
                }
            }
            Event::Text(text) => {
                if in_code_block {
                    code_content.push_str(&text);
                } else {
                    current_text.push_str(&text);
                }
            }
            Event::Start(Tag::Paragraph) => {
                if !current_text.is_empty() {
                    elements.push(view! {
                        <p class="mb-2">{current_text.clone()}</p>
                    });
                    current_text.clear();
                }
            }
            Event::End(Tag::Paragraph) => {
                if !current_text.is_empty() {
                    elements.push(view! {
                        <p class="mb-2">{current_text.clone()}</p>
                    });
                    current_text.clear();
                }
            }
            Event::Start(Tag::Strong) => {
                current_text.push_str("<strong>");
            }
            Event::End(Tag::Strong) => {
                current_text.push_str("</strong>");
            }
            Event::Start(Tag::Emphasis) => {
                current_text.push_str("<em>");
            }
            Event::End(Tag::Emphasis) => {
                current_text.push_str("</em>");
            }
            Event::Start(Tag::Code) => {
                current_text.push_str("<code class=\"bg-gray-200 px-1 rounded\">");
            }
            Event::End(Tag::Code) => {
                current_text.push_str("</code>");
            }
            _ => {}
        }
    }
    
    if !current_text.is_empty() {
        elements.push(view! {
            <p class="mb-2">{current_text}</p>
        });
    }
    
    elements
}

#[component]
fn CodeBlock(language: String, content: String) -> impl IntoView {
    let copy_code = move |_| {
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Some(navigator) = window.navigator().clipboard() {
                    let _ = navigator.write_text(&content).await;
                }
            }
        });
    };

    view! {
        <div class="relative bg-gray-900 rounded-lg p-4 mb-4">
            <div class="flex items-center justify-between mb-2">
                <span class="text-sm text-gray-400">{language}</span>
                <button
                    on:click=copy_code
                    class="text-gray-400 hover:text-white transition-colors"
                    title="Copy code"
                >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                    </svg>
                </button>
            </div>
            <pre class="text-sm text-gray-100 overflow-x-auto">
                <code>{content}</code>
            </pre>
        </div>
    }
} 