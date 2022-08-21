#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use config::UserConfig;
use tauri::Manager;

mod aws;
mod config;
mod error;
mod files;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            get_files,
            get_home_dir,
            list_profiles,
            set_profile,
            get_selected_profile,
            list_buckets,
            list_objects,
            get_objects,
            delete_objects,
            create_folder,
            put_object,
            list_favorite,
            update_favorite,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_config() -> UserConfig {
    config::read_config()
}

#[tauri::command]
fn save_config(download_dir: String, dir_zip: bool) {
    config::save_config(download_dir, dir_zip);
}

#[tauri::command]
fn get_files(dir_name: &str) -> Vec<files::GetFileResponse> {
    println!("data: {}", dir_name);

    let mut target = dir_name.to_string();
    // ディレクトリの最終文字が`/`であった場合は除去しておく
    let char = target.as_str().chars();
    let last = format!("{}", char.clone().nth(char.count() - 1).unwrap());
    if last == "/" && target.len() != 1 {
        let _str = &target[..(target.len() - 1)];
        target = format!("{}", _str);
    }

    // ファイル一覧を取得
    match files::in_directory(&target) {
        Ok(data) => data
            .into_iter()
            .map(|x| files::GetFileResponse {
                is_dir: x.metadata.is_dir(),
                name: x.name,
                file_size: x.size,
                permission: x.permission,
                user: x.user,
                group: x.group,
                update_at: x.date.format("%Y/%m/%d %H:%M").to_string(),
            })
            .collect(),
        Err(_) => Vec::new(),
    }
}

#[tauri::command]
fn get_home_dir() -> Option<String> {
    match dirs::home_dir() {
        Some(path) => Some(path.to_str().unwrap().to_string()),
        None => None,
    }
}

#[tauri::command]
fn list_profiles() -> Vec<String> {
    aws::list_profile_name()
}

#[tauri::command]
fn set_profile(profile: String) -> String {
    aws::set_profile(profile.clone());
    profile
}

#[tauri::command]
fn get_selected_profile() -> Option<String> {
    aws::get_selected_profile()
}

#[tauri::command]
async fn list_buckets() -> Result<Vec<aws::s3::S3Bucket>, String> {
    match aws::s3::list_buckets().await {
        Ok(list) => Ok(list),
        Err(err) => Err(err.name()),
    }
}

#[tauri::command]
async fn list_objects(bucket_name: String, prefix: Option<String>) -> Vec<aws::s3::S3Object> {
    match aws::s3::list_objects(bucket_name.to_string(), prefix).await {
        Ok(list) => list,
        Err(_) => Vec::new(),
    }
}

#[tauri::command]
async fn get_objects(
    bucket_name: String,
    objects: Vec<aws::s3::S3OperationObject>,
) -> Result<Vec<String>, String> {
    // 返却用のリスト
    let mut result = Vec::<String>::new();

    for obj in objects {
        // ダウンロード対象がフォルダかどうかで処理を変更する
        let res = if obj.is_folder {
            aws::s3::get_folder_object(bucket_name.clone(), obj).await
        } else {
            aws::s3::get_object(bucket_name.clone(), obj).await
        };

        match res {
            Ok(p) => {
                result.push(p);
            }
            Err(err) => {
                // エラーがあった場合はメッセージとして返却するが、
                // 他のダウンロードは継続する
                println!("{}", err);
            }
        }
    }
    Ok(result)
}

#[tauri::command]
async fn delete_objects(
    bucket_name: String,
    objects: Vec<aws::s3::S3OperationObject>,
) -> Result<(), String> {
    match aws::s3::delete_objects(bucket_name, objects).await {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{}", err)),
    }
}

#[tauri::command]
async fn create_folder(bucket_name: String, prefix: String) -> Result<bool, String> {
    match aws::s3::create_folder(bucket_name, prefix).await {
        Ok(result) => Ok(result),
        Err(err) => Err(format!("{}", err)),
    }
}

#[tauri::command]
async fn put_object(
    bucket_name: String,
    prefix: Option<String>,
    file_path: String,
) -> Result<bool, String> {
    match aws::s3::put_object(bucket_name, prefix, file_path).await {
        Ok(result) => Ok(result),
        Err(err) => Err(format!("{}", err)),
    }
}

#[tauri::command]
fn list_favorite() -> Result<Vec<config::favorite::Favorite>, String> {
    Ok(config::list_favorite())
}

#[tauri::command]
fn update_favorite(fav: config::favorite::Favorite, is_add: bool) -> Result<bool, String> {
    let mut conf = config::read_config();
    conf.update_favorite(fav, is_add);
    Ok(true)
}
