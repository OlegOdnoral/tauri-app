use tauri::menu::CheckMenuItem;
use tauri::{
    menu::{CheckMenuItemBuilder, MenuBuilder, MenuItem, MenuItemBuilder, SubmenuBuilder},
    tray::TrayIconBuilder,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let restart = MenuItem::with_id(app, "restart", "Restart", true, None::<&str>)?;

            let handle = app.handle();
            let toggle = MenuItemBuilder::with_id("toggle", "Toggle").build(app)?;
            let check = CheckMenuItemBuilder::new("Mark").build(app)?;
            let submenu = SubmenuBuilder::new(handle, "File")
                .item(&MenuItem::new(handle, "MenuItem 1", true, None::<&str>)?)
                .items(&[
                    &CheckMenuItem::new(handle, "CheckMenuItem 1", true, true, None::<&str>)?,
                    &CheckMenuItem::new(handle, "CheckMenuItem 2", true, true, None::<&str>)?,
                ])
                .separator()
                .cut()
                .copy()
                .paste()
                .separator()
                .text("item2", "MenuItem 2")
                .check("checkitem2", "CheckMenuItem 2")
                .icon(
                    "iconitem2",
                    "IconMenuItem 2",
                    app.default_window_icon().cloned().unwrap(),
                )
                .build()?;

            // let menu = Menu::with_items(app, &[&quit_i])?;
            let menu = MenuBuilder::new(app)
                .items(&[&toggle, &submenu, &check, &restart, &quit_i])
                .build()?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .menu_on_left_click(true)
                .icon(app.default_window_icon().unwrap().clone())
                // .on_menu_event(move |app, event| match event.id().as_ref() {
                //     "toggle" => {
                //         println!("toggle clicked");
                //     }
                //     _ => (),
                // })
                // .on_tray_icon_event(|tray, event| {
                //     if let TrayIconEvent::Click {
                //         button: MouseButton::Left,
                //         button_state: MouseButtonState::Up,
                //         ..
                //     } = event
                //     {
                //         let app = tray.app_handle();
                //         if let Some(webview_window) = app.get_webview_window("main") {
                //             let _ = webview_window.show();
                //             let _ = webview_window.set_focus();
                //         }
                //     }
                // })
                .build(app)?;

            app.set_menu(menu)?;
            app.on_menu_event(move |app, event| match event.id().as_ref() {
                "quit" => app.exit(0),
                "restart" => app.restart(),
                _ => (),
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
