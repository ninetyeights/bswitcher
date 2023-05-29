use crate::types::{Browser, BrowserInfo};
use std::collections::HashMap;
use strum::IntoEnumIterator;
extern crate dirs;

#[cfg(target_os = "macos")]
pub fn get_browser_info(browser: &Browser) -> BrowserInfo {
    let home_dir = dirs::home_dir().unwrap();
    match browser {
        Browser::Chrome => BrowserInfo {
            id: 1,
            name: "Google Chrome".to_string(),
            data_dir: home_dir.join("Library/Application Support/Google/Chrome"),
        },
        Browser::Edge => BrowserInfo {
            id: 2,
            name: "Microsoft Edge".to_string(),
            data_dir: home_dir.join("Library/Application Support/Microsoft Edge"),
        },
        Browser::Brave => BrowserInfo {
            id: 3,
            name: "Brave Browser".to_string(),
            data_dir: home_dir.join("Library/Application Support/BraveSoftware/Brave-Browser"),
        },
        // Browser::Vivaldi => BrowserInfo {
        //     id: 4,
        //     name: "Vivaldi".to_string(),
        //     data_dir: home_dir.join("Library/Application Support/Vivaldi"),
        // },
        Browser::Yandex => BrowserInfo {
            id: 5,
            name: "Yandex".to_string(),
            data_dir: home_dir.join("Library/Application Support/Yandex/YandexBrowser"),
        },
    }
}

#[tauri::command]
pub fn get_browsers() -> Option<HashMap<Browser, BrowserInfo>> {
    let mut browsers: HashMap<Browser, BrowserInfo> = HashMap::new();
    for browser in Browser::iter() {
        let info = get_browser_info(&browser);
        let path_exists = info.data_dir.exists();
        if path_exists {
            browsers.insert(browser, info);
        }
    }
    if browsers.is_empty() {
        None
    } else {
        Some(browsers)
    }
}
