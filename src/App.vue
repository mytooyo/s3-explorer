<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { ref } from 'vue';
import ProfilesComponent from './components/ProfilesComponent.vue';
import SideMenu from './components/SideMenu.vue';
// import LocalExplorer from './pages/LocalExplorer.vue';
import SettingDialog from './components/dialog/SettingDialog.vue';
import S3ExplorerOperation from './components/S3ExplorerOperation.vue';
import S3Explorer from './pages/S3Explorer.vue';
import { AppConfig, AppConfigManager } from './repository/config';
import { S3Objects, S3ProileInfo, S3SelectedObjects } from './repository/s3';

const content = ref<string>('s3');

// Config情報
const conf = ref<AppConfig>({
  profile: null,
  download_dir: null,
  dir_zip: false
});
// Config情報取得
async function getConfig() {
  const mng = new AppConfigManager();
  conf.value = await mng.getConfig();
}
getConfig();

// プロファイルのリスト
const profileList = ref<Array<string>>([]);
// 選択中のプロファイル名
const profile = ref<string | null>(null);

// S3情報
const s3Info = ref<S3ProileInfo>(new S3ProileInfo());
const s3Objects = ref<S3Objects>(new S3Objects(s3Info.value));
// 選択中のアイテムを管理するためのクラス
const selectedObjs = ref<S3SelectedObjects>(
  new S3SelectedObjects(s3Objects.value)
);

const onCLickCallback = (val: string) => {
  content.value = val;
  if (val == 'local') {
    s3Objects.value.bucket = null;
  }
};

// プロファイル変更
const onChangeProfile = (val: string) => {
  invoke<string>('set_profile', { profile: val }).then((res) => {
    profile.value = res;
    s3Objects.value.bucket = null;
  });
};

// プロファイルのリストを取得
function getProfiles() {
  invoke<Array<string>>('list_profiles').then((res) => {
    profileList.value = res;
  });
}
getProfiles();

// 選択中のプロファイル情報を取得
function getSelectedProfile() {
  invoke<string | null>('get_selected_profile').then((res) => {
    profile.value = res;
  });
}
getSelectedProfile();

document.addEventListener(
  'contextmenu',
  function (event) {
    event.preventDefault();
    return false;
  },
  { capture: true }
);

// 設定ダイアログフラグ
const showSettingDialog = ref<boolean>(false);

function onCloseSettingDialog() {
  showSettingDialog.value = false;
  // 再取得しておく
  getConfig();
}

function onShowSettingDialog() {
  showSettingDialog.value = true;
}
</script>

<template>
  <div class="flex flex-row max-h-screen">
    <ProfilesComponent
      :profile="profile"
      :profileList="profileList"
      :onChange="onChangeProfile"
      :onSettings="onShowSettingDialog"
    />
    <div class="slate-300">
      <SideMenu
        :profile="profile"
        :s3Info="s3Info"
        :s3Objects="s3Objects"
        :callback="onCLickCallback"
      />
    </div>
    <!-- <LocalExplorer v-if="content == 'local'" /> -->
    <div class="relative w-full overflow-hidden">
      <div class="w-full h-full overflow-y-auto">
        <S3Explorer
          :s3Info="s3Info"
          :s3Objects="s3Objects"
          :selected="selectedObjs"
        />
      </div>
      <!-- Operation Button area -->
      <div
        class="absolute left-0 right-0 bottom-0 h-20"
        v-if="selectedObjs.checked.length > 0"
      >
        <S3ExplorerOperation :selected="selectedObjs" />
      </div>
    </div>

    <div v-show="showSettingDialog">
      <SettingDialog :config="conf" :onClose="onCloseSettingDialog" />
    </div>
  </div>
</template>

<style>
#app {
  font-family: sans-serif, Avenir, Helvetica, Arial;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  background-color: #cbd5e1;
}
</style>
