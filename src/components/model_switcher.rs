use leptos::*;
use crate::models::*;

#[component]
pub fn ModelSwitcher(
    selected_provider: AIProvider,
    selected_model: String,
    on_change: Callback<(AIProvider, String)>,
) -> impl IntoView {
    let (show_dropdown, set_show_dropdown) = create_signal(false);
    let (available_models, set_available_models) = create_signal(Vec::<String>::new());

    // Load available models for the selected provider
    create_effect(move |_| {
        let provider = selected_provider.get();
        spawn_local(async move {
            match crate::api::get_available_models(provider).await {
                Ok(models) => set_available_models.set(models),
                Err(e) => log::error!("Failed to load models: {}", e),
            }
        });
    });

    let toggle_dropdown = move |_| {
        set_show_dropdown.update(|show| *show = !*show);
    };

    let select_model = move |provider: AIProvider, model: String| {
        on_change.call((provider, model));
        set_show_dropdown.set(false);
    };

    let provider_name = move || {
        match selected_provider.get() {
            AIProvider::Ollama => "Ollama",
            AIProvider::OpenAI => "OpenAI",
            AIProvider::Anthropic => "Anthropic",
            AIProvider::Gemini => "Gemini",
            AIProvider::OpenRouter => "OpenRouter",
        }
    };

    view! {
        <div class="relative">
            <button
                on:click=toggle_dropdown
                class="flex items-center space-x-2 px-4 py-2 bg-gray-100 hover:bg-gray-200 rounded-lg transition-colors"
            >
                <span class="text-sm font-medium text-gray-700">{provider_name}</span>
                <span class="text-xs text-gray-500">"/"</span>
                <span class="text-sm text-gray-700">{selected_model}</span>
                <svg
                    class=move || {
                        if show_dropdown.get() {
                            "w-4 h-4 transform rotate-180"
                        } else {
                            "w-4 h-4"
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
                if show_dropdown.get() {
                    view! {
                        <div class="absolute right-0 mt-2 w-64 bg-white rounded-lg shadow-xl border border-gray-200 z-50">
                            <div class="p-2">
                                // Provider selection
                                <div class="mb-3">
                                    <div class="text-xs font-medium text-gray-500 mb-2">"Provider"</div>
                                    <div class="space-y-1">
                                        {vec![
                                            AIProvider::Ollama,
                                            AIProvider::OpenAI,
                                            AIProvider::Anthropic,
                                            AIProvider::Gemini,
                                            AIProvider::OpenRouter,
                                        ].into_iter().map(|provider| {
                                            let provider_name = match provider {
                                                AIProvider::Ollama => "Ollama",
                                                AIProvider::OpenAI => "OpenAI",
                                                AIProvider::Anthropic => "Anthropic",
                                                AIProvider::Gemini => "Gemini",
                                                AIProvider::OpenRouter => "OpenRouter",
                                            };
                                            
                                            let is_selected = move || selected_provider.get() == provider;
                                            let click_handler = move |_| {
                                                // Get first available model for this provider
                                                if let Some(first_model) = available_models.get().first() {
                                                    select_model(provider, first_model.clone());
                                                }
                                            };
                                            
                                            view! {
                                                <button
                                                    on:click=click_handler
                                                    class=move || {
                                                        if is_selected() {
                                                            "w-full text-left px-2 py-1 text-sm bg-blue-100 text-blue-700 rounded"
                                                        } else {
                                                            "w-full text-left px-2 py-1 text-sm hover:bg-gray-100 rounded"
                                                        }
                                                    }
                                                >
                                                    {provider_name}
                                                </button>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>

                                // Model selection
                                <div>
                                    <div class="text-xs font-medium text-gray-500 mb-2">"Model"</div>
                                    <div class="space-y-1 max-h-32 overflow-y-auto">
                                        {move || {
                                            available_models.get().into_iter().map(|model| {
                                                let is_selected = move || selected_model.get() == model;
                                                let model_clone = model.clone();
                                                let click_handler = move |_| {
                                                    select_model(selected_provider.get(), model_clone.clone());
                                                };
                                                
                                                view! {
                                                    <button
                                                        on:click=click_handler
                                                        class=move || {
                                                            if is_selected() {
                                                                "w-full text-left px-2 py-1 text-sm bg-blue-100 text-blue-700 rounded"
                                                            } else {
                                                                "w-full text-left px-2 py-1 text-sm hover:bg-gray-100 rounded"
                                                            }
                                                        }
                                                    >
                                                        {model}
                                                    </button>
                                                }
                                            }).collect::<Vec<_>>()
                                        }}
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}
        </div>
    }
} 