// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::Value;
use std::fs::File;

use tauri::Manager;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use bswitcher::data::get_browsers;
use bswitcher::types::{Browser, Profile};

#[tauri::command]
fn get_profiles(name: Browser) -> Vec<Profile> {
    let info = get_browsers().unwrap();
    let current_browser = info.get(&name).unwrap();
    let state_path = current_browser.data_dir.join("Local State");
    let file = File::open(state_path).unwrap();
    let state: Value = serde_json::from_reader(file).unwrap();
    let info_cache = state["profile"]["info_cache"].to_owned();

    let mut profiles: Vec<Profile> = Vec::new();
    for (key, value) in info_cache.as_object().unwrap().iter() {
        profiles.push(Profile {
            name: match value["gaia_given_name"].as_str() {
                None => value["name"].as_str().unwrap().to_string(),
                Some(name) => {
                    let _name = value["name"].as_str().unwrap();
                    // println!("{}", name.is_empty());
                    if name.is_empty() || _name == name {
                        _name.into()
                    } else {
                        let mut fullname = String::from(name);
                        fullname.push(' ');
                        fullname.push('(');
                        fullname.push_str(value["name"].as_str().unwrap());
                        fullname.push(')');
                        fullname
                    }
                }
            },
            user_name: value["user_name"].as_str().unwrap().to_string(),
            profile_name: key.to_string(),
            checked: false,
            avatarIcon: value["avatar_icon"].as_str().unwrap().to_string(),
            avatar: match value["gaia_picture_file_name"].as_str() {
                None => None,
                Some(name) => {
                    if name.is_empty() {
                        None
                    } else {
                        Some(current_browser.data_dir.join(key).join(name.to_string()))
                    }
                }
            },
        });
    }
    profiles.sort();
    profiles
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            bswitcher::data::get_browsers,
            get_profiles
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
