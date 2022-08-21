use std::os::unix::prelude::{MetadataExt, PermissionsExt};

use chrono::{DateTime, Local, NaiveDateTime, Utc};
use strmode::strmode;
use users::{Groups, Users, UsersCache};

#[derive(serde::Serialize)]
pub struct GetFileResponse {
    pub is_dir: bool,
    pub name: String,
    pub file_size: u64,
    pub permission: String,
    pub user: String,
    pub group: String,
    pub update_at: String,
}

pub struct FileLine {
    pub entry: std::fs::DirEntry,
    pub metadata: std::fs::Metadata,
    pub permission: String,
    pub user: String,
    pub group: String,
    pub size: u64,
    pub date: DateTime<chrono::Local>,
    pub name: String,
}

/** FileLineの構造体の実装 */
impl FileLine {
    // FileLineを生成
    pub fn new(_entry: std::fs::DirEntry, pwd: &String) -> FileLine {
        let path = _entry.path().display().to_string();
        let metadata = _entry.metadata().unwrap();

        // 所有者、グループ名を取得するためのCacher
        let cache = UsersCache::new();
        let user = cache.get_user_by_uid(metadata.uid()).unwrap();
        let group = cache.get_group_by_gid(metadata.gid()).unwrap();

        // 更新日時
        let native = NaiveDateTime::from_timestamp(metadata.atime(), 0);
        let datetime = DateTime::<Local>::from(DateTime::<Utc>::from_utc(native, Utc));
        // ファイル/ディレクトリ名
        let _name = if pwd == "/" {
            path.replace("/", "")
        } else {
            let from = format!("{}/", pwd.as_str());
            path.replace(from.as_str(), "")
        };

        FileLine {
            entry: _entry,
            metadata: metadata.clone(),
            permission: strmode(metadata.permissions().mode()),
            user: format!("{}", user.name().to_string_lossy()),
            group: format!("{}", group.name().to_string_lossy()),
            size: metadata.size(),
            date: datetime,
            name: _name,
        }
    }
}

// ディレクトリ内のデータを取得
pub fn in_directory(target: &String) -> std::io::Result<Vec<FileLine>> {
    // 対象ディレクトリ内の情報を取得し、`FileData`構造体のリストに変換
    let mut entries: Vec<FileLine> = std::fs::read_dir(&target)?
        .filter(|x| {
            let dir = x.as_ref().unwrap();
            dir.metadata().is_ok()
        })
        .map(|x| FileLine::new(x.unwrap(), &target))
        .filter(|_| {
            // ディレクトリのみ表示の場合
            // if args.directory {
            //     return x.metadata.is_dir();
            // }
            // // ファイルのみ表示の場合
            // if args.file {
            //     return x.metadata.is_file();
            // }
            true
        })
        .collect();

    // ソートする
    entries.sort_by(|x1, x2| {
        // 名前とファイルタイプでソート
        x1.name
            .cmp(&x2.name)
            .then(x2.metadata.is_dir().cmp(&x1.metadata.is_dir()))
    });

    Ok(entries)
}
