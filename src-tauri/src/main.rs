#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod payload;
mod wikilink;
mod config;
mod content;
mod category;
mod history;
use content::Content;
use tauri::Manager;
use wikilink::WikiLink;

#[tauri::command]
fn parse_url(url: String) -> WikiLink {
    WikiLink::parse(url)
}

fn content(wikilink: WikiLink) -> payload::UpdateContentPayload {
    let href = wikilink.href();
    let page = content::Page::new(wikilink);

    payload::UpdateContentPayload {
        href,
        body: page.content(),
        tabs: page.tabs().iter().map(|tab| tab.into()).collect(),
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_ = app.handle();
            app.listen_global("setup", move |_| {
                println!("{:?}", config::setup(&app_.config()));
                app_.emit_all("update-content", content(WikiLink::default())).unwrap();
            });

            let app_ = app.handle();
            app.listen_global("page-transition", move |event| {
                match event.payload() {
                    Some(payload) => {
                        match serde_json::from_str::<payload::PageTransitionPayload>(payload) {
                            Ok(payload) => {
                                app_.emit_all("update-content", content(payload.wikilink)).unwrap();
                            },
                            Err(err) => {
                                app_.emit_all("core-error", payload::CoreErrorPayload { message: format!("{}", err) }).unwrap();
                            }
                        }
                    },
                    None => {
                        app_.emit_all("core-error", payload::CoreErrorPayload { message: "payload error".to_string() }).unwrap();
                    },
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![parse_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
