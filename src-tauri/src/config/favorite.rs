use std::collections::HashMap;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Favorites {
    pub items: HashMap<String, Vec<Favorite>>,
}

// お気に入りアイテム
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Favorite {
    bucket: String,
    prefix: String,
}

impl Favorites {
    pub fn new() -> Self {
        Favorites {
            items: HashMap::new(),
        }
    }

    // お気に入り登録する
    pub fn add_favorite(&mut self, profile: String, fav: Favorite) {
        // プロファイルが存在する場合
        if let Some(list) = self.items.get_mut(&profile.clone()) {
            let exists = list
                .iter()
                .any(|x| x.bucket == fav.bucket && x.prefix == fav.prefix);
            // 既にリストに存在する場合は無視
            if exists {
                println!("already exists favorite: {}, {}", fav.bucket, fav.prefix);
                return;
            }
            list.push(fav);
        } else {
            // プロファイルが存在しない場合は新たに生成
            self.items.insert(profile, vec![fav]);
        }
    }

    // お気に入りから削除
    pub fn remove_favorite(&mut self, profile: String, fav: Favorite) {
        // プロファイルが存在しない場合は無視
        let list_opt = self.items.get_mut(&profile.clone());
        if list_opt.is_none() {
            println!("does not exists profile: {}", profile);
            return;
        }
        let list = list_opt.unwrap();

        let position = list
            .iter()
            .position(|x| x.bucket == fav.bucket && x.prefix == fav.prefix);
        if let Some(p) = position {
            list.remove(p);
        } else {
            println!("does not exists favorite: {}, {}", fav.bucket, fav.prefix);
        }
    }
}

// impl Favorite {
//     fn new(bucket: String, prefix: String) -> Self {
//         Favorite { bucket, prefix }
//     }
// }
