import { invoke } from '@tauri-apps/api/tauri';

// Config情報
export interface AppConfig {
  // 選択中のプロファイル名
  profile: string | null;
  // ダウンロードディレクトリ
  download_dir: string | null;
  // ダウンロード時のZIP化
  dir_zip: boolean;
}

// Config情報の管理用のクラス
export class AppConfigManager {
  conf: AppConfig | null = null;

  // Config情報取得
  async getConfig(): Promise<AppConfig> {
    return await invoke<AppConfig>('get_config');
  }

  // Config情報更新
  async saveConfig(conf: AppConfig) {
    console.log(conf.dir_zip);
    console.log(conf.download_dir);
    await invoke('save_config', {
      downloadDir: conf.download_dir,
      dirZip: conf.dir_zip
    });
  }
}
