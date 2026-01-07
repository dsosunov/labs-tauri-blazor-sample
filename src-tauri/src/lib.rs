// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_prevent_default::init())
        .invoke_handler(tauri::generate_handler![greet])
        .on_page_load(|webview, payload| {
            let url = payload.url();
            println!("[Navigation] {} -> {}", webview.label(), url);

            // On Linux, when we detect navigation to our OAuth callback URL,
            // re-navigate with a marker to bypass WebKit's blocking behavior
            // Skip error responses (e.g., login_required, access_denied)
            #[cfg(target_os = "linux")]
            if url.as_str().starts_with("tauri://localhost/authentication/")
                && !url.as_str().contains("_handled=1")
                && !url.as_str().contains("error=") {
                println!("[Navigation] Detected OAuth callback, re-navigating via WebView API");
                let mut modified_url = url.clone();
                modified_url.set_query(Some(&format!("{}&_handled=1",
                    url.query().unwrap_or(""))));
                let _ = webview.navigate(modified_url);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
