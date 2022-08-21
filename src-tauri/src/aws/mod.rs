use aws_types::SdkConfig;
use std::time::SystemTime;

pub mod profile;
pub mod s3;

/// Get AWS Config
pub async fn aws_config() -> SdkConfig {
    // リージョン名
    let region_name = "ap-northeast-1";

    // 選択中のプロファイルを取得するためconfigファイルを読み込む
    let user_config = super::config::read_config();

    // 選択中のプロファイルが存在する場合はリストから取得
    if let Some(selected) = user_config.profile {
        // プロファイル名をリストから取得
        let p_map = profile::read_profiles();

        // プロファイルを取得
        if let Some(p) = p_map.get(&selected) {
            // awsのクレデンシャルを生成
            let cred = aws_types::Credentials::new(
                p.access_key_id.clone().unwrap(),
                p.secret_access_key.clone().unwrap(),
                p.session_token.clone(),
                None,
                "Statics",
            );

            return aws_config::from_env()
                .region(aws_sdk_sts::Region::new(region_name))
                .credentials_provider(cred)
                .load()
                .await;
        }
    }

    // 存在しない場合は環境情報のデフォルト値から取得
    aws_config::from_env()
        .region(aws_sdk_sts::Region::new(region_name))
        .load()
        .await
}

/// Parse date string from aws datetime
pub fn parse_datetime(datetime: &aws_smithy_types::DateTime) -> String {
    match SystemTime::try_from(*datetime) {
        Ok(d_time) => {
            let d: chrono::DateTime<chrono::Local> = d_time.into();
            d.format("%Y/%m/%d %H:%M:%S").to_string()
        }
        Err(_) => "".to_string(),
    }
}

/// プロファイル名のみを取得する
pub fn list_profile_name() -> Vec<String> {
    let p = profile::read_profiles();
    let mut list = p.iter().map(|x| x.0.clone()).collect::<Vec<String>>();
    list.sort();
    list
}

/// 使用するプロファイルを選択する
pub fn set_profile(p: String) {
    let mut user_config = super::config::read_config();

    // プロファイル名を更新
    user_config.profile = Some(p);

    // 書き込み
    user_config.write_to_file();
}

/// 選択中のプロファイルを取得
pub fn get_selected_profile() -> Option<String> {
    let user_config = super::config::read_config();
    user_config.profile.clone()
}
