use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use strum_macros::EnumIter;

#[derive(Debug, Serialize, Deserialize, EnumIter, Eq, Hash, PartialEq)]
pub enum Browser {
    Chrome,
    Edge,
    Brave,
    // Vivaldi,
    Yandex
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BrowserInfo {
    pub id: u32,
    pub name: String,
    pub data_dir: PathBuf
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct Profile {
    pub name: String,
    pub user_name: String,
    pub profile_name: String,
    pub checked: bool,
    pub avatar: Option<PathBuf>,
    pub avatarIcon: String
}