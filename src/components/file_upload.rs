use leptos::*;
use crate::models::*;
use web_sys::FileList;

#[component]
pub fn FileUpload(on_upload: Callback<Vec<FileUpload>>) -> impl IntoView {
    let file_input_ref = create_node_ref::<html::Input>();

    let handle_file_select = move |_| {
        if let Some(input) = file_input_ref.get() {
            if let Some(files) = input.files() {
                spawn_local(async move {
                    let mut uploaded_files = Vec::new();
                    
                    for i in 0..files.length() {
                        if let Some(file) = files.get(i) {
                            if let Ok(array_buffer) = file.array_buffer().await {
                                if let Ok(bytes) = js_sys::Uint8Array::new(&array_buffer).to_vec() {
                                    let file_upload = FileUpload {
                                        name: file.name(),
                                        content_type: file.type_(),
                                        data: bytes,
                                    };
                                    uploaded_files.push(file_upload);
                                }
                            }
                        }
                    }
                    
                    if !uploaded_files.is_empty() {
                        on_upload.call(uploaded_files);
                    }
                });
            }
        }
    };

    let trigger_file_select = move |_| {
        if let Some(input) = file_input_ref.get() {
            let _ = input.click();
        }
    };

    view! {
        <div>
            <input
                ref=file_input_ref
                type="file"
                multiple=true
                accept="image/*,application/pdf,text/*"
                class="hidden"
                on:change=handle_file_select
            />
            <button
                type="button"
                on:click=trigger_file_select
                class="p-2 text-gray-500 hover:text-gray-700 transition-colors"
                title="Attach files"
            >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13"></path>
                </svg>
            </button>
        </div>
    }
} 