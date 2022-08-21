use std::{
    collections::HashMap,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use aws_sdk_s3::{
    error::ListBucketsError,
    model::{Delete, ObjectIdentifier},
    types::{ByteStream, SdkError},
};
use platform_dirs::UserDirs;
use tokio_stream::StreamExt;

use crate::{
    config::{self, UserConfig},
    error::aws_error::{AwsError, AwsErrorKind},
};

#[derive(serde::Serialize)]
pub struct S3Bucket {
    pub name: String,
    pub created_at: String,
    pub location: String,
}

#[derive(serde::Serialize)]
pub struct S3Object {
    pub key: String,
    pub last_modified: Option<String>,
    pub size: Option<i64>,
    pub storage_class: Option<String>,
    pub is_folder: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct S3OperationObject {
    pub prefix: String,
    pub is_folder: bool,
}

// クライアント生成
async fn init_client() -> aws_sdk_s3::Client {
    // aws config
    let config = super::aws_config().await;

    // S3 client
    aws_sdk_s3::Client::new(&config)
}

/// S3 list buckets
pub async fn list_buckets() -> Result<Vec<S3Bucket>, AwsError> {
    // S3 client
    let client = init_client().await;

    // S3 list bucket request
    let res = client.list_buckets().send().await;
    if res.is_err() {
        let error: SdkError<ListBucketsError> = res.err().unwrap();
        // エラーの種類によって返却するエラーを変更する
        let kind = match error {
            SdkError::ConstructionFailure(_) => AwsErrorKind::AccessDenied,
            SdkError::TimeoutError(_) => AwsErrorKind::AccessDenied,
            SdkError::DispatchFailure(_) => AwsErrorKind::AccessDenied,
            SdkError::ResponseError { err: _, raw: _ } => AwsErrorKind::AccessDenied,
            SdkError::ServiceError { err, raw: _ } => {
                if err.code() == Some("ExpiredToken") {
                    AwsErrorKind::ExpiredToken
                } else {
                    AwsErrorKind::AccessDenied
                }
            }
        };
        return Err(AwsError::new(kind));
    }

    // レスポンスからバケット一覧を取得
    let result = res.unwrap();
    let buckets = result.buckets().unwrap_or_default();

    // 返却するリスト
    let mut list = Vec::<S3Bucket>::new();

    // 取得した情報を整形
    for bucket in buckets {
        // 必要な値が設定されている場合のみ追加
        if let (Some(name), Some(creation_date)) = (bucket.name(), bucket.creation_date()) {
            // バケットの詳細情報を取得
            // ロケーション情報
            let r = match client
                .get_bucket_location()
                .bucket(name.clone())
                .send()
                .await
            {
                Ok(v) => v,
                Err(_) => return Err(AwsError::new(AwsErrorKind::NotFoundObject)),
            };

            let location = r.location_constraint().unwrap().as_str();

            let created_at = super::parse_datetime(creation_date);
            list.push(S3Bucket {
                name: name.to_string(),
                created_at,
                location: location.to_string(),
            });
        }
    }

    Ok(list)
}

/// 指定のパケットのオブジェクト一覧を取得
pub async fn list_objects(
    bucket_name: String,
    prefix: Option<String>,
) -> Result<Vec<S3Object>, aws_sdk_s3::Error> {
    // S3 client
    let client = init_client().await;

    // バケットインスタンスを生成
    let mut bucket = client
        .list_objects_v2()
        .bucket(bucket_name.clone())
        .delimiter("/");

    // プレフィックスの指定がある場合は設定
    if let Some(p) = &prefix {
        bucket = bucket.prefix(p);
    }

    // リクエスト
    let mut stream = bucket.into_paginator().send();
    // let objects = result.contents().unwrap_or_default();

    // 取得した情報を格納するリスト
    let mut list = Vec::<S3Object>::new();

    // 置換するプレフィックス
    let replace_prefix = match prefix {
        Some(v) => {
            println!("prefix?: {}", v);
            if v.len() == 0 {
                None
            } else {
                if v.ends_with("/") {
                    Some(v)
                } else {
                    let r = format!("{}/", v);
                    Some(r)
                }
            }
        }
        None => None,
    };

    // 設定済みの階層を格納しておく
    let mut setted = HashMap::<String, bool>::new();

    while let Some(res) = stream.next().await {
        if res.is_err() {
            break;
        }
        let data = res.unwrap();
        let objects = data.contents().unwrap_or_default();

        // 取得した情報を整形
        for obj in objects {
            // 必要な情報が設定されている場合のみ実施
            if let (Some(key), Some(last_modified), Some(storage_class)) =
                (obj.key(), obj.last_modified(), obj.storage_class())
            {
                // プレフィックスが指定されている場合はその部分を除去
                let key_prefix = if let Some(p) = &replace_prefix {
                    key.replace(p.as_str(), "")
                } else {
                    key.to_string()
                };

                println!("key: {}", key_prefix);

                // 置換した結果、空となった場合は無視
                if key_prefix.is_empty() {
                    continue;
                }

                // 区切り文字でsplitした最初のアイテムのみ返却する
                let item = key_prefix.split("/").into_iter().collect::<Vec<&str>>();

                // プレフィックスが指定されている場合は1番目の値を取得する
                let data = item[0].to_string();

                // アイテムを生成する
                let mut s3_object = S3Object {
                    key: data.to_string(),
                    last_modified: Some(super::parse_datetime(&last_modified)),
                    size: Some(obj.size),
                    storage_class: Some(storage_class.as_str().to_string()),
                    is_folder: false,
                };

                // キーの一番最後の値は"/"の場合はディレクトリと判定する
                if key_prefix.ends_with("/") && obj.size == 0 && setted.get(&data).is_none() {
                    s3_object.is_folder = true;
                    list.push(s3_object);
                    setted.insert(data, true);
                    continue;
                }

                // ディレクトリ内のデータの場合は無視
                if item.len() > 1 {
                    // ディレクトリの第一階層の場合は追加しておく
                    if setted.get(&data).is_none() {
                        s3_object.is_folder = true;
                        list.push(s3_object);
                        setted.insert(data, true);
                    }
                    continue;
                }

                list.push(s3_object);
            }
        }

        // フォルダについては`delimiter`を設定すると`common_prefix`に含まれるため、
        // そこから取得する
        let cm_prefix = data.common_prefixes.unwrap_or_default();
        for obj in cm_prefix {
            // プレフィックスが設定されている場合のみ
            if let Some(p) = obj.prefix {
                // プレフィックスが指定されている場合はその部分を除去
                let key_prefix = if let Some(rp) = &replace_prefix {
                    p.replace(rp.as_str(), "")
                } else {
                    p.to_string()
                };

                let s3_object = S3Object {
                    key: key_prefix.clone(),
                    last_modified: None,
                    size: None,
                    storage_class: None,
                    is_folder: true,
                };

                if setted.get(&key_prefix).is_none() {
                    list.push(s3_object);
                    setted.insert(key_prefix, true);
                    continue;
                }
            }
        }
    }

    Ok(list)
}

// 指定のオブジェクトを削除
pub async fn delete_objects(
    bucket_name: String,
    objects: Vec<S3OperationObject>,
) -> Result<(), aws_sdk_s3::Error> {
    // S3 client
    let client = init_client().await;

    // 削除対象のリストを作成
    let mut dels: Vec<ObjectIdentifier> = vec![];
    for obj in objects {
        // フォルダであった場合は中のリストを取得
        if obj.is_folder {
            // バケットインスタンスを生成
            let bucket = client
                .list_objects_v2()
                .bucket(bucket_name.clone())
                .prefix(obj.prefix);
            // 指定パス内の情報を取得
            let res = bucket.send().await?;
            let objs = res.contents().unwrap_or_default();
            for o in objs {
                let obj_id = ObjectIdentifier::builder().set_key(o.key.clone()).build();
                dels.push(obj_id);
            }
        } else {
            let obj_id = ObjectIdentifier::builder()
                .set_key(Some(obj.prefix))
                .build();
            dels.push(obj_id);
        }
    }

    // 削除リクエスト作成
    let delete = Delete::builder().set_objects(Some(dels.clone())).build();

    // リクエスト
    client
        .delete_objects()
        .bucket(bucket_name)
        .delete(delete)
        .send()
        .await?;

    println!("objects deleted. completed!: {:?}", dels);

    Ok(())
}

// 指定のオブジェクトをダウンロード
pub async fn get_object(
    bucket_name: String,
    object: S3OperationObject,
) -> Result<String, aws_sdk_s3::Error> {
    // S3 client
    let client = init_client().await;

    // オブジェクトを取得
    let res = client
        .get_object()
        .bucket(bucket_name)
        .key(&object.prefix)
        .send()
        .await?;

    // プレフィックスからファイル名を取得
    // 区切り文字でsplitした最後のアイテムのみ返却する
    let item = object.prefix.split("/").into_iter().collect::<Vec<&str>>();
    let file_name = item[item.len() - 1];

    // Configファイル読み込み
    let conf = config::read_config();

    // 指定がないユーザディレクトリからダウンロードディレクトリを取得
    let dwn_file = __download_path(&conf, None).join(file_name);

    let _dwn_file = save_file(res.body, false, dwn_file).await;

    Ok(_dwn_file.as_path().to_str().unwrap().to_string())
}

fn __download_path(conf: &UserConfig, dir_name: Option<String>) -> PathBuf {
    // 指定がないユーザディレクトリからダウンロードディレクトリを取得
    match &conf.download_dir {
        Some(v) => {
            // ディレクトリの指定がある場合はそれを追加
            if let Some(dir_name) = dir_name {
                PathBuf::from(v).join(dir_name)
            } else {
                PathBuf::from(v)
            }
        }
        None => {
            let user_dirs = UserDirs::new().unwrap();
            // ディレクトリの指定がある場合はそれを追加
            if let Some(dir_name) = dir_name {
                user_dirs.download_dir.join(dir_name)
            } else {
                user_dirs.download_dir
            }
        }
    }
}

// ファイルを保存
async fn save_file(body: ByteStream, is_folder: bool, path: PathBuf) -> PathBuf {
    let body = body.collect().await.unwrap().into_bytes();

    // 保存するパス
    let mut p = path.clone();

    println!("download file: {:?}", p.as_path());

    // フォルダの場合はダウンロード先のディレクトリを作成
    if is_folder {
        std::fs::create_dir_all(p.parent().unwrap()).expect("could not create directory");
    }
    // ファイルが存在するか確認し、存在する場合は別名のファイルを作成
    if p.exists() {
        loop {
            // 存在しなくなるまでファイルを生成する
            let new_p = new_name(p);
            p = new_p.new_path;
            if !p.exists() {
                break;
            }
        }
    }

    // ファイルを作成
    let mut file = std::fs::File::create(&p).expect("unable to create file");
    let result = file.write_all(&body.to_vec());
    if result.is_err() {
        println!("file write error: {:?}", result.err());
    }

    return p;
}

// フォルダ内のオブジェクトをすべてダウンロード
pub async fn get_folder_object(
    bucket_name: String,
    object: S3OperationObject,
) -> Result<String, aws_sdk_s3::Error> {
    // Configファイル読み込み
    let conf = config::read_config();

    // S3 client
    let client = init_client().await;

    // バケットインスタンスを生成
    let bucket = client
        .list_objects_v2()
        .bucket(bucket_name.clone())
        .prefix(&object.prefix);
    // 指定パス内の情報を取得
    let res = bucket.send().await?;
    let objs = res.contents().unwrap_or_default();

    // 指定されたプレフィックスが複数階層の場合は最後部分のみ取得して
    // それをディレクトリとする
    let item = object.prefix.split("/").into_iter().collect::<Vec<&str>>();
    let mut dir_name = if object.prefix.ends_with("/") {
        if item.len() > 1 {
            item[item.len() - 2].to_string()
        } else {
            item[item.len() - 1].to_string()
        }
    } else {
        item[item.len() - 1].to_string()
    };

    // ダウンロードディレクトリを取得
    let np = __download_path(&conf, None);

    // ZIP圧縮対象の場合はダミーのディレクトリ名を設定する
    let save_dir = if conf.dir_zip {
        let mut new_dir_name = dir_name.clone();
        let mut tmp = np.join(dir_name.clone());
        // ディレクトリが存在する場合は別のディレクトリ名を作成
        if tmp.exists() {
            loop {
                let new_p = new_name(tmp);
                tmp = new_p.new_path;
                new_dir_name = new_p.new_name;
                if !tmp.exists() {
                    break;
                }
            }
        }
        // ディレクトリ名を更新
        dir_name = new_dir_name;
        tmp
    } else {
        np.join(dir_name.clone())
    };

    // 指定のオブジェクトを1つずつ保存する
    for obj in objs {
        if let Some(key) = obj.key.clone() {
            // キーの一番最後の値が"/"の場合はディレクトリのため取得しない
            if key.ends_with("/") {
                continue;
            }

            // オブジェクトを取得
            let out = client
                .get_object()
                .bucket(bucket_name.clone())
                .key(key.clone())
                .send()
                .await?;

            // ファイル名取得
            let s = &key.split("/").into_iter().collect::<Vec<&str>>();
            let file_name = s[s.len() - 1];

            // ディレクトリを取得
            let mut target_dir = save_dir.clone();
            let key_prefix = &key.replace(&object.prefix, "");
            let kps = key_prefix.split("/").into_iter().collect::<Vec<&str>>();
            if kps.len() > 1 {
                for i in 0..(kps.len() - 1) {
                    target_dir = target_dir.clone().join(kps[i]);
                    // ディレクトリを作成
                    std::fs::create_dir_all(target_dir.clone())
                        .expect("error create sub directory");
                }
            }

            // 指定がないユーザディレクトリからダウンロードディレクトリを取得
            let dwn_file = target_dir.join(file_name);
            println!("saveed: {:?}", dwn_file);

            // ファイル保存
            let dwn_file = save_file(out.body, true, dwn_file).await;

            println!("downloaded: {:?}", dwn_file)
        }
    }

    // ZIPが必要な場合はZIP圧縮
    if conf.dir_zip {
        // ZIPファイルを生成
        let zip_file_path = __download_path(&conf, Some(format!("{}.zip", &dir_name)));
        let zip_file = std::fs::File::create(&zip_file_path).unwrap();

        // ディレクトリを取得
        let dir = save_dir.clone();

        // ディレクトリの親パス名を取得
        let parent_dir = dir.parent().unwrap().to_str().unwrap().to_string();

        // 追加するディレクトリのパス文字列
        let add_dir = dir.as_path().to_str().unwrap().to_string();

        // ディレクトリ内探索を行うためのWalkDirを使用
        let walkdir = walkdir::WalkDir::new(&add_dir);

        // ZIPファイル生成
        let mut zip = zip::ZipWriter::new(zip_file);

        // 対象のディレクトリ内のファイルに対して処理
        let mut buffer = Vec::new();
        for entry in walkdir.into_iter().filter_map(|x| x.ok()) {
            let path = entry.path();
            let name = path
                .strip_prefix(Path::new(&parent_dir))
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap();

            // ファイル書き込み時のオプション
            let mut options = zip::write::FileOptions::default();

            // ファイルの場合は読み込んでデータを追加
            if path.is_file() {
                println!("adding file {:?} as {:?} ...", path, name);

                // ファイル読み込み
                let mut f = std::fs::File::open(path).expect("file open error");
                f.read_to_end(&mut buffer).expect("file read error");

                // ファイルのメタデータから更新日時を取得
                if let Ok(metadata) = f.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        // ローカルタイムに一旦変換
                        let local_dt: chrono::DateTime<chrono::Local> = modified.into();
                        let ts = local_dt.naive_local().timestamp();
                        let t = time::OffsetDateTime::from_unix_timestamp(ts).unwrap();
                        if let Ok(date_time) = zip::DateTime::from_time(t) {
                            options = options.last_modified_time(date_time);
                        }
                    }
                }

                // ファイル書き込みを行う
                zip.start_file(name, options).expect("start file error");

                // ZIPファイルに書き込み
                zip.write_all(&*buffer).expect("file write error");

                // バッファをクリア
                buffer.clear();
            } else {
                println!("adding dir {:?} as {:?} ...", path, name);
                zip.add_directory(name, options)
                    .expect("error zip add directory");
            }
        }

        // ZIPファイルクローズ
        zip.flush().expect("error zip flush");

        // ZIP化する前のディレクトリを削除
        let _ = std::fs::remove_dir_all(save_dir);
    }

    Ok(object.prefix)
}

// フォルダを作成
pub async fn create_folder(bucket_name: String, prefix: String) -> Result<bool, aws_sdk_s3::Error> {
    // S3 client
    let client = init_client().await;

    // PutPbjectインスタンス生成
    let req = client.put_object().bucket(bucket_name.clone()).key(prefix);
    req.send().await?;

    Ok(true)
}

// ファイルをアップロード
pub async fn put_object(
    bucket_name: String,
    prefix: Option<String>,
    file_path: String,
) -> Result<bool, aws_sdk_s3::Error> {
    // S3 client
    let client = init_client().await;

    // PutPbjectインスタンス生成
    let mut req = client.put_object().bucket(bucket_name.clone());

    // ファイル名を取得
    let path_buf = PathBuf::from(&file_path);
    let file_name = path_buf.file_name().unwrap().to_str().unwrap();

    // プレフィックスの指定がある場合は設定
    if let Some(p) = prefix {
        if p.len() > 0 {
            req = req.key(format!("{}/{}", p, file_name));
        } else {
            req = req.key(file_name);
        }
    } else {
        req = req.key(file_name);
    }

    // アップロードするファイルのバイナリを生成
    match ByteStream::from_path(Path::new(file_path.as_str())).await {
        Ok(b) => {
            req = req.body(b);
            // ファイルアップロードリクエスト
            req.send().await?;
            println!("uploaded: file!");
        }
        Err(err) => {
            println!("file open error: {}", err);
            return Ok(false);
        }
    }

    Ok(true)
}

struct NewPathName {
    new_path: PathBuf,
    new_name: String,
}

// ファイル名のチェックおよびファイル名生成
// 同一のファイル名が存在する場合は新しいファイルを付与
fn new_name(base_path: PathBuf) -> NewPathName {
    let user_dirs = PathBuf::from(base_path.parent().clone().unwrap());

    let name = base_path.file_name().unwrap().to_str().unwrap();
    let new_name: String;

    // ファイル名と拡張子を分けて別名を付与する
    if let Some(ext) = base_path.extension() {
        let r = format!(".{}", ext.to_str().unwrap());
        let replaced = name.replace(r.as_str(), "");
        new_name = format!("{}_copy.{}", replaced, ext.to_str().unwrap()).clone();
    } else {
        // 拡張子がない場合はそのまま後ろに追加
        new_name = format!("{}_copy", name).clone();
    }

    // 新たなファイルを返却
    NewPathName {
        new_path: user_dirs.join(new_name.clone()),
        new_name: new_name.clone(),
    }
}
