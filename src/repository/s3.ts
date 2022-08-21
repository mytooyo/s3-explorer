import { invoke } from '@tauri-apps/api/tauri';

// S3Bucket
export interface S3BucketInterface {
  name: string;
  created_at: string;
  location: string;
}

// S3 favorite
export interface S3Favorite {
  bucket: string;
  prefix: string;
}

export class S3ProileInfo {
  buckets: Array<S3BucketInterface> = [];
  favorites: Array<S3Favorite> = [];
  errored: boolean = false;

  listBuckets() {
    invoke<Array<S3BucketInterface>>('list_buckets')
      .then((res) => {
        this.buckets = res;
        this.errored = false;

        this.listFavorite();
      })
      .catch((e) => {
        this.buckets = [];
        this.errored = true;
      });
  }

  // 指定のバケットが存在するか確認
  checkBucket(b: string | null): boolean {
    for (const bucket of this.buckets) {
      if (bucket.name == b) {
        return true;
      }
    }
    return false;
  }

  listFavorite() {
    invoke<Array<S3Favorite>>('list_favorite').then((res) => {
      this.favorites = res;
    });
  }

  // お気に入り更新
  updateFavorite(
    bucket: string,
    prefix: string,
    isNew: boolean,
    completion: Function
  ) {
    const fav: S3Favorite = {
      bucket: bucket,
      prefix: prefix
    };
    console.log(fav);

    invoke<boolean>('update_favorite', {
      fav: fav,
      isAdd: isNew
    }).then((res) => {
      this.listFavorite();
      completion();
    });
  }

  // お気に入りかを判定
  isFavorite(s3Objects: S3Objects, displayPath: string | null): boolean {
    for (const fav of this.favorites) {
      if (fav.bucket == s3Objects.bucket && fav.prefix == displayPath) {
        return true;
      }
    }
    return false;
  }
}

// S3 Object item for rust interface
export interface S3ObjectInterface {
  key: string;
  last_modified: string | null;
  size: number | null;
  storage_class: string | null;
  is_folder: boolean;
}

// S3オブジェクトダウンロード,削除時のRustとのインタフェース
interface S3OperationObject {
  prefix: string;
  is_folder: boolean;
}

// S3 Objects in bucket
interface S3ObjectsInterface {
  bucket: string | null;
  prefix: string | null;
  list: Array<S3ObjectInterface>;
}

export class S3Objects implements S3ObjectsInterface {
  s3Info: S3ProileInfo;
  bucket: string | null = null;
  prefix: string | null = null;

  list: S3ObjectInterface[] = [];

  refetch: boolean = true;

  // コンストラクタ
  constructor(s3Info: S3ProileInfo) {
    this.s3Info = s3Info;
  }

  // 指定のバケットが存在するか確認
  checkBucket(b: string | null): boolean {
    return this.s3Info.checkBucket(b);
  }

  // バケット名を更新
  updateBucket(b: string | null) {
    this.bucket = b;
    this.prefix = null;
    this.refetch = true;
  }

  // プレフィックス(フォルダ)を更新
  updatePrefix(p: string | null) {
    this.prefix = p;
    this.refetch = true;
  }

  // バケット名とプレフィックス(フォルダ)を更新
  update(bucket: string | null, prefix: string | null) {
    this.updateBucket(bucket);
    this.updatePrefix(prefix);
  }

  // プレフィックス(フォルダパス)を追加
  pushPrefix(p: string) {
    if (this.prefix) {
      this.prefix = this.prefix + p;
    } else {
      this.prefix = p;
    }
    this.refetch = true;
  }

  // 強制アップデート
  forceUpdate() {
    this.refetch = true;
  }

  // アイテム選択
  onSelected(item: S3ObjectInterface) {
    // フォルダタイプの場合
    if (item.is_folder) {
      // 既にプレフィックスが指定済みの場合は追加
      if (this.prefix) {
        this.pushPrefix(item.key);
      } else {
        this.updatePrefix(item.key);
      }
      return;
    }
    // ファイルタイプの場合は選択状態にする
  }

  // バケット情報を取得
  listObjects(callback: Function) {
    if (!this.bucket || !this.refetch) {
      return;
    }
    invoke<Array<S3ObjectInterface>>('list_objects', {
      bucketName: this.bucket,
      prefix: this.prefix
    }).then((res) => {
      this.refetch = false;
      this.list = res;
      callback(res);
    });
  }

  // オブジェクトをダウンロード
  getObjects(objects: S3OperationObject[], callback: Function) {
    // ダウンロード
    invoke<Array<S3ObjectInterface>>('get_objects', {
      bucketName: this.bucket,
      objects: objects
    }).then((_) => {
      callback();
    });
  }

  // オブジェクトアップロード
  putObject(filePath: string) {
    invoke<boolean>('put_object', {
      bucketName: this.bucket!,
      prefix: this.prefix,
      filePath: filePath
    }).then((result) => {
      // 正常に登録が完了した場合は再取得
      if (result) {
        this.forceUpdate();
      }
    });
  }

  // オブジェクトを削除
  removeObjects(objects: S3OperationObject[], callback: Function) {
    // 削除リクエスト
    invoke<Array<S3ObjectInterface>>('delete_objects', {
      bucketName: this.bucket,
      objects: objects
    }).then((res) => {
      callback();
      // 再取得を行うためにフラグを立てる
      this.refetch = true;
    });
  }

  // フォルダ作成
  createFolder(folderName: string, callback: Function) {
    // 最後にスラッシュがついているか確認
    const isLastSlash = folderName.substring(folderName.length - 1) == '/';

    // S3に登録するキー情報を設定
    var key = folderName + (isLastSlash ? '' : '/');
    if (this.prefix) {
      key = this.prefix + '/' + folderName + (isLastSlash ? '' : '/');
    }
    // 作成リクエスト
    invoke<boolean>('create_folder', {
      bucketName: this.bucket,
      prefix: key
    }).then((result) => {
      // 正常に登録が完了した場合は再取得
      if (result) {
        this.forceUpdate();
      }
      callback();
    });
  }
}

export class S3SelectedObjects {
  // 選択済みのリスト(インデックス)
  checked: Array<number> = [];
  // ダウンロード中を示すフラグ
  downloading: boolean = false;
  // ダウンロード完了を示すフラグ
  downloaded: boolean = false;
  // ダイアログ表示を示すフラグ
  showedDialog: boolean = false;

  // 表示中のS3オブジェクト情報
  s3Objects: S3Objects;

  // コンストラクタ
  constructor(s3Objects: S3Objects) {
    this.s3Objects = s3Objects;
  }

  // チェックボックス選択時の処理
  onCheckBox(index: number) {
    // 存在しない場合
    if (!this.checked.includes(index)) {
      this.checked.push(index);
    }
    // 存在する場合
    else {
      const i = this.checked.indexOf(index);
      this.checked.splice(i, 1);
    }
  }

  // チェックされているプレフィックス(フォルダ)を返却
  getCheckedItems(): Array<S3OperationObject> {
    var items: Array<S3OperationObject> = [];
    // チェックされたアイテムを取得
    for (const index of this.checked) {
      console.log(index);
      // オブジェクト取得
      const obj = this.s3Objects.list[index];

      var p = obj.key;
      if (this.s3Objects.prefix) {
        // 最後にスラッシュがついているか確認
        const pp = this.s3Objects.prefix;
        const isLastSlash = pp.substring(pp.length - 1) == '/';
        if (isLastSlash) {
          p = pp + obj.key;
        } else {
          p = pp + '/' + obj.key;
        }
      }

      items.push({
        prefix: p,
        is_folder: obj.is_folder
      });
    }
    return items;
  }

  // 選択中のオブジェクトをダウンロード
  download() {
    // フラグを立てる
    this.downloading = true;

    // 選択中のアイテム取得
    const items = this.getCheckedItems();
    console.log(items);

    // ダウンロード処理
    this.s3Objects.getObjects(items, () => {
      // ダウンロード中のフラグを消す
      this.downloading = false;
      // ダウンロード済みのフラグを立てる
      this.downloaded = true;

      // 指定秒数の間メッセージを表示し、その後非表示
      setTimeout(() => {
        this.downloaded = false;
        this.checked = [];
      }, 3000);
    });
  }

  // 削除確認ダイアログ表示
  showConfirmDialog() {
    this.showedDialog = true;
  }

  // 削除確認ダイアログ非表示
  onCloseConfirmDialog() {
    this.showedDialog = false;
  }

  // 選択中のオブジェクトを削除
  delete() {
    // 選択中のアイテム取得
    const items = this.getCheckedItems();

    // 削除処理を実施
    this.s3Objects.removeObjects(items, () => {
      this.checked = [];
      this.showedDialog = false;
    });
  }
}
