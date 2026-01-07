// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn handler(webview: &tauri::Webview, event: &tauri::WebviewEvent) {
    // Log all webview events to catch navigation at the lowest level
    let webview_label = webview.label();

    println!("[Navigation] Webview '{}' event: {:?}", webview_label, event);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_prevent_default::init())
        .invoke_handler(tauri::generate_handler![greet])
        .on_webview_event(handler)
        .on_page_load(|webview, payload| {
            // Log page loads with URL
            println!("[Navigation] Page loaded: {} in webview '{}'", payload.url(), webview.label());
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
