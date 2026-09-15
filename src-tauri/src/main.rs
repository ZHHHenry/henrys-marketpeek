#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod quotes;

use std::sync::Mutex;
use std::time::{Duration, Instant};

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Emitter, Manager, PhysicalPosition, WebviewWindow, WindowEvent};

static LAST_SAVE: Mutex<Option<Instant>> = Mutex::new(None);

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_main(app);
        }))
        .invoke_handler(tauri::generate_handler![
            quotes::get_instruments,
            quotes::fetch_quotes,
            hide_window,
            quit_app,
            set_language
        ])
        .setup(|app| {
            let win = app.get_webview_window("main").expect("main window");
            place_window(&win);
            let _ = win.show();
            setup_tray(app)?;
            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let _ = window.hide();
                let _ = window.app_handle().emit("mp-hidden", ());
            }
            WindowEvent::Moved(pos) => {
                save_position_throttled(window.app_handle(), *pos);
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("failed to run MarketPeek");
}

#[tauri::command]
fn hide_window(app: tauri::AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.hide();
        let _ = app.emit("mp-hidden", ());
    }
}

#[tauri::command]
fn quit_app(app: tauri::AppHandle) {
    app.exit(0);
}

#[tauri::command]
fn set_language(app: tauri::AppHandle, lang: String) {
    if let Ok(menu) = build_menu(&app, &lang) {
        if let Some(tray) = app.tray_by_id("main") {
            let _ = tray.set_menu(Some(menu));
        }
    }
}

fn build_menu(app: &tauri::AppHandle, lang: &str) -> tauri::Result<Menu<tauri::Wry>> {
    let (show_label, quit_label) = if lang == "en" {
        ("Show", "Quit")
    } else {
        ("显示", "退出")
    };
    let show = MenuItem::with_id(app, "show", show_label, true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", quit_label, true, None::<&str>)?;
    Menu::with_items(app, &[&show, &quit])
}

fn show_main(app: &tauri::AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.show();
        let _ = win.set_focus();
        let _ = app.emit("mp-shown", ());
    }
}

fn setup_tray(app: &mut tauri::App) -> tauri::Result<()> {
    let menu = build_menu(app.handle(), "zh")?;

    let tray_icon = tauri::image::Image::from_bytes(include_bytes!("../icons/tray.png"))?;

    let builder = TrayIconBuilder::with_id("main")
        .tooltip("Henry's MarketPeek")
        .icon(tray_icon)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => show_main(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(win) = app.get_webview_window("main") {
                    if win.is_visible().unwrap_or(false) {
                        let _ = win.hide();
                        let _ = app.emit("mp-hidden", ());
                    } else {
                        let _ = win.show();
                        let _ = win.set_focus();
                        let _ = app.emit("mp-shown", ());
                    }
                }
            }
        });

    builder.build(app)?;
    Ok(())
}

fn config_path(app: &tauri::AppHandle) -> Option<std::path::PathBuf> {
    app.path().app_config_dir().ok().map(|d| d.join("window.json"))
}

fn place_window(win: &WebviewWindow) {
    if let Some(path) = config_path(win.app_handle()) {
        if let Ok(txt) = std::fs::read_to_string(&path) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&txt) {
                if let (Some(x), Some(y)) = (
                    v.get("x").and_then(|v| v.as_i64()),
                    v.get("y").and_then(|v| v.as_i64()),
                ) {
                    let _ = win.set_position(PhysicalPosition::new(x as i32, y as i32));
                    return;
                }
            }
        }
    }
    if let Ok(Some(mon)) = win.primary_monitor() {
        let scale = win.scale_factor().unwrap_or(1.0);
        let size = win.outer_size().unwrap_or(tauri::PhysicalSize::new(420, 320));
        let msize = mon.size();
        let mpos = mon.position();
        let margin = (16.0 * scale) as i32;
        let x = mpos.x + msize.width as i32 - size.width as i32 - margin;
        let y = mpos.y + margin;
        let _ = win.set_position(PhysicalPosition::new(x, y));
    }
}

fn save_position_throttled(app: &tauri::AppHandle, pos: PhysicalPosition<i32>) {
    {
        let mut last = match LAST_SAVE.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let now = Instant::now();
        if let Some(t) = *last {
            if now.duration_since(t) < Duration::from_millis(400) {
                return;
            }
        }
        *last = Some(now);
    }
    if let Some(path) = config_path(app) {
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let _ = std::fs::write(path, serde_json::json!({ "x": pos.x, "y": pos.y }).to_string());
    }
}
