#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod quotes;

use std::sync::Mutex;
use std::time::{Duration, Instant};
use std::{fs, io::Write};

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Emitter, Manager, PhysicalPosition, WebviewWindow, WindowEvent};

struct PositionSaveState {
    last_save: Option<Instant>,
    latest: Option<PhysicalPosition<i32>>,
}

static POSITION_STATE: Mutex<PositionSaveState> = Mutex::new(PositionSaveState {
    last_save: None,
    latest: None,
});

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_main(app);
        }))
        .invoke_handler(tauri::generate_handler![
            quotes::get_instruments,
            quotes::fetch_quotes,
            quotes::fetch_custom_quotes,
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
    save_latest_position(&app);
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
            "quit" => {
                save_latest_position(app);
                app.exit(0);
            }
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
    app.path()
        .app_config_dir()
        .ok()
        .map(|d| d.join("window.json"))
}

fn place_window(win: &WebviewWindow) {
    let size = win
        .outer_size()
        .unwrap_or(tauri::PhysicalSize::new(420, 320));
    if let Some(path) = config_path(win.app_handle()) {
        if let Ok(txt) = std::fs::read_to_string(&path) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&txt) {
                if let (Some(x), Some(y)) = (
                    v.get("x").and_then(|v| v.as_i64()),
                    v.get("y").and_then(|v| v.as_i64()),
                ) {
                    if let (Ok(x), Ok(y)) = (i32::try_from(x), i32::try_from(y)) {
                        let position = PhysicalPosition::new(x, y);
                        if position_is_visible(win, position, size) {
                            let _ = win.set_position(position);
                            return;
                        }
                    }
                }
            }
        }
    }
    if let Ok(Some(mon)) = win.primary_monitor() {
        let scale = win.scale_factor().unwrap_or(1.0);
        let msize = mon.size();
        let mpos = mon.position();
        let margin = (16.0 * scale) as i32;
        let x = mpos.x + msize.width as i32 - size.width as i32 - margin;
        let y = mpos.y + margin;
        let _ = win.set_position(PhysicalPosition::new(x, y));
    }
}

fn position_is_visible(
    win: &WebviewWindow,
    position: PhysicalPosition<i32>,
    size: tauri::PhysicalSize<u32>,
) -> bool {
    let Ok(monitors) = win.available_monitors() else {
        return false;
    };
    const MIN_VISIBLE: i64 = 80;
    let left = i64::from(position.x);
    let top = i64::from(position.y);
    let right = left + i64::from(size.width);
    let bottom = top + i64::from(size.height);
    monitors.iter().any(|monitor| {
        let monitor_position = monitor.position();
        let monitor_size = monitor.size();
        rectangle_has_minimum_overlap(
            (left, top, right, bottom),
            (
                i64::from(monitor_position.x),
                i64::from(monitor_position.y),
                i64::from(monitor_position.x) + i64::from(monitor_size.width),
                i64::from(monitor_position.y) + i64::from(monitor_size.height),
            ),
            MIN_VISIBLE,
        )
    })
}

fn rectangle_has_minimum_overlap(
    window: (i64, i64, i64, i64),
    monitor: (i64, i64, i64, i64),
    minimum: i64,
) -> bool {
    let visible_width = window.2.min(monitor.2) - window.0.max(monitor.0);
    let visible_height = window.3.min(monitor.3) - window.1.max(monitor.1);
    visible_width >= minimum && visible_height >= minimum
}

fn save_position_throttled(app: &tauri::AppHandle, pos: PhysicalPosition<i32>) {
    let should_save = {
        let mut state = lock_position_state();
        state.latest = Some(pos);
        let now = Instant::now();
        if let Some(t) = state.last_save {
            if now.duration_since(t) < Duration::from_millis(400) {
                false
            } else {
                state.last_save = Some(now);
                true
            }
        } else {
            state.last_save = Some(now);
            true
        }
    };
    if should_save {
        write_position(app, pos);
    }
}

fn save_latest_position(app: &tauri::AppHandle) {
    let latest = lock_position_state().latest;
    if let Some(position) = latest {
        write_position(app, position);
    }
}

fn lock_position_state() -> std::sync::MutexGuard<'static, PositionSaveState> {
    POSITION_STATE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn write_position(app: &tauri::AppHandle, pos: PhysicalPosition<i32>) {
    if let Some(path) = config_path(app) {
        if let Some(dir) = path.parent() {
            if let Err(error) = fs::create_dir_all(dir) {
                eprintln!("failed to create position config directory: {error}");
                return;
            }
        }
        let data = serde_json::json!({ "x": pos.x, "y": pos.y }).to_string();
        match fs::File::create(&path).and_then(|mut file| {
            file.write_all(data.as_bytes())?;
            file.sync_all()
        }) {
            Ok(()) => {}
            Err(error) => eprintln!("failed to save window position: {error}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rectangle_has_minimum_overlap;

    #[test]
    fn window_position_requires_a_usable_visible_area() {
        let monitor = (0, 0, 1920, 1080);
        assert!(rectangle_has_minimum_overlap(
            (1500, 50, 1920, 370),
            monitor,
            80
        ));
        assert!(rectangle_has_minimum_overlap(
            (-340, 50, 80, 370),
            monitor,
            80
        ));
        assert!(!rectangle_has_minimum_overlap(
            (-341, 50, 79, 370),
            monitor,
            80
        ));
        assert!(!rectangle_has_minimum_overlap(
            (2000, 50, 2420, 370),
            monitor,
            80
        ));
    }
}
