use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{MediaRecorder, MediaRecorderOptions, Blob};

#[component]
pub fn VoiceInput() -> impl IntoView {
    let (is_recording, set_is_recording) = create_signal(false);
    let (transcript, set_transcript) = create_signal(String::new());

    let start_recording = move |_| {
        set_is_recording.set(true);
        set_transcript.set(String::new());
        
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Some(navigator) = window.navigator().media_devices() {
                    if let Ok(stream) = navigator.get_user_media_with_constraints(&js_sys::Object::new()).await {
                        // For now, we'll just show a placeholder
                        // In a real implementation, you'd use the MediaRecorder API
                        set_transcript.set("Voice recording started...".to_string());
                    }
                }
            }
        });
    };

    let stop_recording = move |_| {
        set_is_recording.set(false);
        // In a real implementation, you'd stop the recording and process the audio
        set_transcript.set("Voice recording stopped".to_string());
    };

    view! {
        <div class="relative">
            <button
                type="button"
                on:click=move |_| {
                    if is_recording.get() {
                        stop_recording(());
                    } else {
                        start_recording(());
                    }
                }
                class=move || {
                    if is_recording.get() {
                        "p-2 text-red-500 hover:text-red-700 transition-colors"
                    } else {
                        "p-2 text-gray-500 hover:text-gray-700 transition-colors"
                    }
                }
                title="Voice input"
            >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"></path>
                </svg>
            </button>
            
            // Recording indicator
            {move || {
                if is_recording.get() {
                    view! {
                        <div class="absolute -top-1 -right-1 w-3 h-3 bg-red-500 rounded-full animate-pulse"></div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}
        </div>
    }
} 