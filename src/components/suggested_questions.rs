use leptos::*;
use crate::models::*;

#[component]
pub fn SuggestedQuestions(
    questions: Vec<SuggestedQuestion>,
    on_question_click: Callback<String>,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-lg p-4 mb-6">
            <h3 class="text-lg font-semibold text-gray-800 mb-3">"Suggested Questions"</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
                {questions.into_iter().map(|question| {
                    let question_text = question.question.clone();
                    let click_handler = move |_| {
                        on_question_click.call(question_text.clone());
                    };
                    
                    view! {
                        <button
                            on:click=click_handler
                            class="p-3 text-left bg-gray-50 hover:bg-gray-100 rounded-lg border border-gray-200 transition-colors"
                        >
                            <span class="text-sm text-gray-700">{question.question}</span>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
} 