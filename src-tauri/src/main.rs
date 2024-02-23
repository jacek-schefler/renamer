// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use regex::Regex;
use std::{
    borrow::Borrow,
    fs::rename,
    io,
    path::{Path, PathBuf},
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![change_names])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn change_names(
    comment: String,
    errand: String,
    folder: String,
    add_numbers: bool,
) -> Result<(), String> {
    let comment_normalized = normalize_comment(&comment, &errand);
    let gcode_files = find_gcode_files(&folder).map_err(|err| err.to_string())?;
    let re_to_remove = Regex::new(r".*]\s?").map_err(|err| err.to_string())?;
    let re_numbers = Regex::new(r"(?i).*toolpath(\d+)").map_err(|err| err.to_string())?;
    for mut file in gcode_files {
        let old_file_name = file.clone();
        let name = file
            .file_stem()
            .ok_or(String::from("Problem z odczytem nazwy scieżki"))?
            .to_str()
            .ok_or(String::from("Problem z odczytem nazwy scieżki"))?;
        let ext = file
            .extension()
            .ok_or(String::from("Problem z odczytaniem rozszerzenia pliku"))?
            .to_str()
            .ok_or(String::from("Problem z odczytaniem rozrzeszenia pliku"))?;
        let tmp_name = normalize(re_to_remove.replace(name, "").borrow());
        let tmp_number = re_numbers
            .captures(name)
            .map_or("", |c| c.get(1).map_or("", |m| m.as_str()));
        let new_name = match (add_numbers, comment_normalized.len() > 0) {
            (true, true) => format!(
                "{:0>2}-{} {}.{}",
                tmp_number, tmp_name, comment_normalized, ext
            ),
            (true, false) => format!("{:0>2}-{}.{}", tmp_number, tmp_name, ext),
            (false, true) => format!("{} {}.{}", tmp_name, comment_normalized, ext),
            (false, false) => format!("{}.{}", tmp_name, ext),
        };
        file.set_file_name(new_name);
        rename(old_file_name, file).map_err(|err| err.to_string())?;
    }
    Ok(())
}

fn normalize_comment(comment: &str, errand: &str) -> String {
    let mut tmp = comment
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || c == &' ')
        .collect::<String>()
        .to_uppercase();
    tmp.truncate(20);
    format!("{}{}", errand, tmp)
}

fn normalize(name: &str) -> String {
    let mut tmp = name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || c == &' ' || c == &'-' || c == &'.')
        .collect::<String>()
        .replace("--", "-")
        .replace("  ", " ")
        .to_uppercase();
    tmp.truncate(20);
    tmp
}
fn find_gcode_files(folder: &str) -> io::Result<Vec<PathBuf>> {
    let folder_path = Path::new(&folder);
    let files: Vec<PathBuf> = folder_path
        .read_dir()?
        .filter_map(|f| f.ok())
        .filter(|f| match f.path().extension() {
            None => false,
            Some(ex) => ex == "tap" || ex == "nc" || ex == "las",
        })
        .map(|f| f.path())
        .collect();
    Ok(files)
}
