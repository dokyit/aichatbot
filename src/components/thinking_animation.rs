use leptos::*;

#[component]
pub fn ThinkingAnimation() -> impl IntoView {
    view! {
        <div class="flex justify-start">
            <div class="max-w-3xl rounded-lg p-4 bg-gray-100">
                <div class="flex items-center space-x-1">
                    <span class="text-sm text-gray-600">"AI is thinking"</span>
                    <div class="flex space-x-1">
                        <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce"></div>
                        <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 0.1s;"></div>
                        <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 0.2s;"></div>
                    </div>
                </div>
            </div>
        </div>
    }
} 