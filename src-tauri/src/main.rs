// Prevents an extra console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

/// Returns true when `url` belongs to the app itself and may load inside
/// the window: the Tauri production scheme (`tauri://localhost/...` on
/// macOS), the Windows production origin (`http://tauri.localhost/...`),
/// and inline schemes. Everything else is not the app's own content.
fn is_app_url(url: &tauri::Url) -> bool {
    match url.scheme() {
        "tauri" => true,
        "http" | "https" => url.host_str() == Some("tauri.localhost"),
        "about" | "data" | "blob" => true,
        _ => false,
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let nav_handle = app.handle().clone();
            let new_window_handle = app.handle().clone();
            WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                .title("Weather Everywhere")
                .inner_size(1200.0, 850.0)
                .on_navigation(move |url| {
                    if is_app_url(url) {
                        // The workshop itself. It is one page whose rooms
                        // are in-page anchors, so moving around the
                        // workshop — atlas, labs, notebook — resolves to
                        // the app's own URL and stays inside the window.
                        true
                    } else if url.scheme() == "http" || url.scheme() == "https" {
                        // An outside page: a "Go deeper" source (NOAA,
                        // NASA, BAS, Open-Meteo) or any other external
                        // link. Cancel the in-app navigation and hand the
                        // URL to the OS default browser instead. The
                        // workshop stays exactly where it was.
                        let _ = nav_handle.opener().open_url(url.as_str(), None::<&str>);
                        false
                    } else {
                        true
                    }
                })
                .on_new_window(move |url, _features| {
                    // An external link that asks for its own window gets
                    // the same treatment as an external navigation: never
                    // open a second in-app window; send the URL to the
                    // OS default browser.
                    if url.scheme() == "http" || url.scheme() == "https" {
                        let _ = new_window_handle
                            .opener()
                            .open_url(url.as_str(), None::<&str>);
                    }
                    tauri::webview::NewWindowResponse::Deny
                })
                .on_download(|_webview, _event| {
                    // Downloads are the workshop's own business: the
                    // notebook exports its JSON through a download.
                    // Allow every download with the platform's default
                    // behavior (the webview saves under the suggested
                    // file name) instead of blocking it.
                    true
                })
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Weather Everywhere");
}
