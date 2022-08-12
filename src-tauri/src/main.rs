#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod wikilink;
use wikilink::WikiLink;

#[tauri::command]
async fn main_content() -> String {
    "<h1 id=\"content-head\">head</h1>
     <div id=\"content-body\">body</div>".into()
}

#[tauri::command]
fn parse_url(url: String) -> Result<WikiLink, String> {
    Ok(WikiLink::parse(url).unwrap_or(WikiLink::default()))
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![main_content, parse_url])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
