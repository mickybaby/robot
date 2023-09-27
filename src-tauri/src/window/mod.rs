use crate::APP;
use tauri::Menu;

#[cfg(target_os = "macos")]
pub fn create_external_windows(label: String, title: String, external_url: String) {
    let app = APP.get().unwrap();
    tauri::WindowBuilder::new(
        app,
        label, /* the unique window label */
        tauri::WindowUrl::External(external_url.parse().unwrap()),
    )
    .min_inner_size(1000.00, 600.00)
    .title(title)
    .decorations(true)
    .build()
    .expect("failed to build window");
}

#[cfg(not(target_os = "macos"))]
pub fn create_external_windows(label: String, title: String, external_url: String) {
    let app = APP.get().unwrap();
    tauri::WindowBuilder::new(
        app,
        label, /* the unique window label */
        tauri::WindowUrl::External(external_url.parse().unwrap()),
    )
    .min_inner_size(1000.00, 600.00)
    .title(title)
    .menu(Menu::new())
    .build()
    .expect("failed to build window");
}
