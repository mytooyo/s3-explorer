use std::io::Write;

pub mod favorite;

// JSONのデフォルト値
fn default_dir_zip() -> bool {
    false
}
fn default_download_dir() -> Option<String> {
    use platform_dirs::UserDirs;
    let user_dirs = UserDirs::new().unwrap();
    let dwn_path = user_dirs.download_dir.as_path();
    Some(dwn_path.as_os_str().to_str().unwrap().to_string())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserConfig {
    // 選択中のプロファイル情報
    pub profile: Option<String>,

    // お気に入りリスト
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorites: Option<favorite::Favorites>,

    // ダウンロード先のディレクトリ
    #[serde(default = "default_download_dir")]
    pub download_dir: Option<String>,

    // フォルダダウンロード時にZIP圧縮するか否か
    #[serde(default = "default_dir_zip")]
    pub dir_zip: bool,
}

impl Default for UserConfig {
    fn default() -> Self {
        use platform_dirs::UserDirs;
        let user_dirs = UserDirs::new().unwrap();
        let dwn_path = user_dirs.download_dir.as_path();
        Self {
            profile: None,
            favorites: None,
            download_dir: Some(dwn_path.as_os_str().to_str().unwrap().to_string()),
            dir_zip: false,
        }
    }
}

// Configファイルパスを取得
fn __get_pathbuf() -> std::path::PathBuf {
    // ホームディレクトリを取得
    let opt_home = dirs::home_dir();

    // configディレクトリを取得
    let mut conf_dir = opt_home.unwrap();
    conf_dir.push(".config");

    // 存在しない場合は生成
    if !conf_dir.exists() {
        std::fs::create_dir_all(&conf_dir).expect("could not create config directory");
    }

    // ファイル追加
    let mut conf_file = conf_dir;
    conf_file.push("s3-explorer.json");

    conf_file
}

// Configファイル読み込み
pub fn read_config() -> UserConfig {
    let conf_file = __get_pathbuf();

    // ファイルが存在しない場合はデフォルト返却
    if !conf_file.exists() {
        return UserConfig::default();
    }

    // 存在する場合は読み込み
    let f = std::fs::File::open(conf_file);
    // 読み込みエラーの場合はデフォルト返却
    if f.is_err() {
        return UserConfig::default();
    }

    // ファイルリーダー
    let reader = std::io::BufReader::new(f.unwrap());
    // jsonを構造体に展開
    serde_json::from_reader(reader).unwrap()
}

// 選択中のプロファイルのお気に入りリストを返却
pub fn list_favorite() -> Vec<favorite::Favorite> {
    let conf = read_config();

    // 選択中のプロファイルが存在しないまたは
    // お気に入りが未設定の場合は空のリストを返却
    if conf.profile.is_none() || conf.favorites.is_none() {
        return vec![];
    }

    // 選択中のプロファイル
    let profile = conf.profile.unwrap();

    // プロファイルと一致するリストを取得
    if let Some(favorites) = conf.favorites.unwrap().items.get(&profile) {
        return favorites.to_vec();
    }

    // 存在しない場合は空のリストを返却
    vec![]
}

// Config情報保存
pub fn save_config(download_dir: String, dir_zip: bool) {
    // Configファイルを読み込んで値を更新
    let mut conf = read_config();
    conf.download_dir = Some(download_dir);
    conf.dir_zip = dir_zip;

    // ファイル書き込み
    conf.write_to_file();
}

// UserConfigに対する処理
impl UserConfig {
    // Configファイル書き込み
    pub fn write_to_file(&self) {
        // ファイルパスを取得
        let conf_file = __get_pathbuf();

        // ファイル作成
        let f = std::fs::File::create(conf_file);

        // 作成できない場合はエラー
        if f.is_err() {
            return;
        }

        // テキストに変換
        let text = serde_json::to_string_pretty(self).unwrap();

        // 書き込み
        let _ = f.unwrap().write(text.as_bytes());
    }

    // お気に入り登録
    pub fn update_favorite(&mut self, fav: favorite::Favorite, is_add: bool) {
        let profile = self.profile.clone();
        if profile.is_none() {
            return;
        }

        println!("update favorite: {:?}, {}", profile, is_add);

        // 追加の場合
        if is_add {
            // お気に入りが未登録の場合は新たに生成
            if self.favorites.is_none() {
                self.favorites = Some(favorite::Favorites::new());
            }
            let list = self.favorites.as_mut().unwrap();
            list.add_favorite(profile.unwrap(), fav);
        } else {
            // 削除モードの場合
            if self.favorites.is_none() {
                return;
            }
            let list = self.favorites.as_mut().unwrap();
            list.remove_favorite(profile.unwrap(), fav);
        }
        self.write_to_file();
    }
}
