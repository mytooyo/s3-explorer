use std::{
    collections::HashMap,
    io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Write},
};

pub const CONFIG_FILE_NAME: &str = "config";
pub const CREDENTIAL_FILE_NAME: &str = "credentials";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AWSProfile {
    pub name: String,
    pub region: String,
    pub output: String,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
    pub session_token: Option<String>,
}

pub trait AWSConfigManager<T> {
    fn new(val: HashMap<String, HashMap<String, String>>) -> Self;
}

struct Config {
    region: String,
    output: String,
}

struct Configs {
    items: HashMap<String, Config>,
}

impl AWSConfigManager<Config> for Configs {
    fn new(val: HashMap<String, HashMap<String, String>>) -> Self {
        let mut items = HashMap::<String, Config>::new();
        for (key, ele) in val {
            items.insert(
                key.clone().replace("profile ", ""),
                Config {
                    region: ele["region"].clone(),
                    output: ele["output"].clone(),
                },
            );
        }
        Configs { items }
    }
}

#[derive(Clone, Debug)]
struct Credential {
    access_key_id: Option<String>,
    secret_access_key: Option<String>,
    session_token: Option<String>,
}

struct Credentials {
    items: HashMap<String, Credential>,
}

impl Credentials {
    /// HashMapからOption型で取り出す
    fn get_value_from_map(map: &HashMap<String, String>, key: &str) -> Option<String> {
        map.get(key).cloned()
    }
}

impl AWSConfigManager<Credential> for Credentials {
    fn new(val: HashMap<String, HashMap<String, String>>) -> Self {
        let mut items = HashMap::<String, Credential>::new();
        for (key, ele) in val {
            items.insert(
                key.clone().replace("profile ", ""),
                Credential {
                    access_key_id: Self::get_value_from_map(&ele, "aws_access_key_id"),
                    secret_access_key: Self::get_value_from_map(&ele, "aws_secret_access_key"),
                    session_token: Self::get_value_from_map(&ele, "aws_session_token"),
                },
            );
        }
        Credentials { items }
    }
}

// AWS Config, Credentialファイル読み込み
pub fn read_profiles() -> HashMap<String, AWSProfile> {
    // configファイル読み込み
    let configs = match read::<Configs, Config>(CONFIG_FILE_NAME) {
        Ok(v) => v.items,
        Err(_) => HashMap::<String, Config>::new(),
    };

    // credentialファイル読み込み
    let credentials = match read::<Credentials, Credential>(CREDENTIAL_FILE_NAME) {
        Ok(v) => v.items,
        Err(_) => HashMap::<String, Credential>::new(),
    };

    // 返却用のリストを生成
    let mut list = HashMap::<String, AWSProfile>::new();

    // configファイルから生成する
    for (key, val) in configs {
        // credentialの情報を取得
        let (mut access_key_id, mut secret_access_key, mut session_token): (
            Option<String>,
            Option<String>,
            Option<String>,
        ) = (None, None, None);

        // 同一プロファイル名のCredentialを取得
        if let Some(cred) = credentials.get(&key) {
            access_key_id = cred.access_key_id.clone();
            secret_access_key = cred.secret_access_key.clone();
            session_token = cred.session_token.clone();
        }
        list.insert(
            key.clone(),
            AWSProfile {
                name: key,
                region: val.region,
                output: val.output,
                access_key_id,
                secret_access_key,
                session_token,
            },
        );
    }

    list
}

/// ファイル読み込み
pub fn read<T, S>(file_name: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: AWSConfigManager<S>,
{
    // ホームディレクトリを取得
    let opt_home = dirs::home_dir();
    // ホームディレクトリが存在しない場合は作成するかの確認メッセージを表示
    if opt_home.is_none() {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "Oops... does not home directory..orz",
        )));
    }
    // awsディレクトリを生成
    let mut aws_dir = opt_home.unwrap();
    aws_dir.push(".aws");

    // 存在確認を行い、存在しない場合は作成するかの確認メッセージを表示
    if !aws_dir.exists() {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "Oops... does not [~/.aws] directory..orz",
        )));
    }

    // ファイルのパスを生成
    let mut config_path = aws_dir;
    config_path.push(file_name);
    // ファイルの存在確認し、存在しない場合は空で作成しておく
    if !config_path.exists() {
        let mut f = BufWriter::new(std::fs::File::create(config_path.clone())?);
        f.write_all("".as_bytes())?;
    }

    // ファイルを読み込み
    let file = std::fs::File::open(config_path)?;
    let reader = BufReader::new(file);
    // 読み込んだファイルをHashMap形式に整形
    let result = shape_aws_toml(reader)?;
    Ok(T::new(result))
}

/// AWS関連のファイルデータをMap形式に整形する
fn shape_aws_toml(
    reader: BufReader<std::fs::File>,
) -> std::io::Result<HashMap<String, HashMap<String, String>>> {
    // 読み込んだデータ用のマップ
    let mut tomls = HashMap::new();

    // プロファイルの名称を取得するための正規表現
    let re = regex::Regex::new(r"\[(.*)\]").unwrap();

    // 現在処理中のキー名と格納用マップ
    let mut key_name: Option<String> = None;

    // バッファから1行ずつ読み込んで処理する
    for line in reader.lines() {
        // ライン読み込み
        let l = line?;

        // トリムした結果、空行であった場合は無視
        if l.as_str().trim().is_empty() {
            continue;
        }

        // 名称の行と一致する場合
        if let Some(name) = re.captures(l.as_str()) {
            let key = name.get(1).unwrap().as_str().to_string();
            key_name = Some(key.clone());
            tomls.insert(key, "".to_string());
            continue;
        }

        // `=`で分割する
        let c: char = '=';
        let data = l.split(c).collect::<Vec<&str>>();
        // 分割できなかった場合は無視
        if data.len() < 2 {
            continue;
        }
        let key = key_name.clone().unwrap();
        if let Some(x) = tomls.get_mut(&key) {
            // 未設定状態の場合はそのまま設定
            *x = if x.is_empty() {
                data.join("=")
            } else {
                let v = vec![x.clone(), data.join("=")];
                v.join(";")
            }
        }
    }

    // 返却用のMapを生成
    let mut ret_map = HashMap::new();
    // 読み込んだデータのキー分実施
    for (key, val) in tomls.iter() {
        // キー内の要素を格納するMap
        let mut data_map = HashMap::new();
        let c: char = '=';
        let c2: char = ';';
        // セミコロンで分割することでキー要素内の個々のデータを取得する
        val.split(c2).for_each(|x| {
            // 個々のデータをKey, Value形式に変換してMapに追加
            let v = x
                .split(c)
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();
            // Base64変換された認証情報は値に`=`が含まれるため、すべてをjoinするようにする
            let s = v[1..(v.len())].to_vec().join("=");
            data_map.insert(v[0].clone(), s);
        });
        ret_map.insert(key.clone(), data_map);
    }

    Ok(ret_map)
}
