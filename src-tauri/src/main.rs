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
                .title("Travel Kit")
                .inner_size(1280.0, 900.0)
                .on_navigation(move |url| {
                    if is_app_url(url) {
                        // The course itself: its pages (index, discovery,
                        // START-HERE) and in-page lesson anchors resolve to
                        // the app's own URLs and stay inside the window.
                        true
                    } else if url.scheme() == "http" || url.scheme() == "https" {
                        // An outside page: a documentation link, a
                        // Discovery Room recording, or any other external
                        // destination. Cancel the in-app navigation and
                        // hand the URL to the OS default browser instead.
                        // The course stays exactly where it was.
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
                    // Downloads are the course's own business: "Save
                    // progress to USB" exports a JSON file, and the
                    // workbook is a PDF. Allow every download with the
                    // platform's default behavior instead of blocking it.
                    true
                })
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Travel Kit");
}
