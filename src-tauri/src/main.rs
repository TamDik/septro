#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod category;
mod config;
mod content;
mod history;
mod payload;
mod utils;
mod wiki;
mod wikilink;
mod markdown;
use tauri::Manager;
use std::sync::Mutex;
use wiki::Wiki;
use wikilink::WikiLink;

#[tauri::command]
fn parse_url(url: String) -> WikiLink {
    WikiLink::parse(url)
}

fn content(wiki: &Wiki, wikilink: WikiLink) -> payload::UpdateContent {
    let href = wikilink.href();
    let content = wiki.get_content(wikilink);

    payload::UpdateContent {
        href,
        body: content.content(),
        tabs: content.tabs().iter().map(|tab| tab.into()).collect(),
        scripts: content.scripts().iter().map(|script| script.into()).collect(),
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = tauri::api::path::app_dir(&app.config()).unwrap();
            let wiki = Mutex::new(Wiki::from_master_config_file(&app_dir).unwrap_or_else(|_| Wiki::new(&app_dir)));
            app.manage(wiki);

            let app_ = app.handle();
            app.listen_global("setup", move |_| {
                let wiki: tauri::State<Mutex<Wiki>> = app_.state();
                let wiki = wiki.lock().unwrap();
                app_.emit_all("update-content", content(&wiki, WikiLink::default())).unwrap();
            });

            let app_ = app.handle();
            app.listen_global("page-transition", move |event| {
                match event.payload() {
                    Some(payload) => {
                        match serde_json::from_str::<payload::PageTransition>(payload) {
                            Ok(payload) => {
                                let wiki: tauri::State<Mutex<Wiki>> = app_.state();
                                let wiki = wiki.lock().unwrap();
                                app_.emit_all("update-content", content(&wiki, payload.wikilink)).unwrap();
                            },
                            Err(err) => {
                                app_.emit_all("core-error", payload::CoreError { message: format!("{}", err) }).unwrap();
                            }
                        }
                    },
                    None => {
                        app_.emit_all("core-error", payload::CoreError { message: "payload error".to_string() }).unwrap();
                    },
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![parse_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
