<script setup lang="ts">
import { open } from '@tauri-apps/api/dialog';
import { ref, toRefs, watch } from 'vue';
import { AppConfig, AppConfigManager } from '../../repository/config';
import ToggleSwitch from '../atoms/ToggleSwitch.vue';

const props = defineProps<{
  config: AppConfig;
  onClose: Function;
}>();

// 入力フィールドのID名
const dlDirContainerId = 'dl-dir-container-id';
const dlDirFieldId = 'dl-dir-field-id';

const { config } = toRefs(props);

// 入力フィールドのデフォルト値
const fieldValue = ref<string | null>(config.value.download_dir);

// ZIP圧縮のデフォルト値
var zipValue = ref<boolean>(config.value.dir_zip ?? false);
var zipUpdateValue = zipValue.value;
var onChangedZipValue = false;

// 監視
watch(config, () => {
  if (!fieldValue.value) {
    fieldValue.value = config.value.download_dir;
  }
  if (!onChangedZipValue) {
    zipValue.value = config.value.dir_zip;
  }
});

// フォルダ選択
const openFolder = async () => {
  // フォルダの選択ダイアログを開く
  const selected = await open({
    directory: true,
    multiple: false,
    defaultPath: fieldValue.value!
  });

  if (Array.isArray(selected)) {
    return;
  } else {
    fieldValue.value = selected;
  }
};

// ZIPのトグル変更時の処理
function onChangeZipValue(val: boolean) {
  zipUpdateValue = val;
  onChangedZipValue = true;
}

// 設定情報保存
async function saveConfig() {
  // 入力フィールドから値を取得
  const input = document.getElementById(dlDirFieldId) as HTMLInputElement;
  if (!input) {
    return;
  }

  // AppConfigのオブジェクトをコピーして値を更新
  const c = Object.assign({}, config.value);
  c.download_dir = input.value;
  c.dir_zip = zipUpdateValue;

  // 保存
  const mng = new AppConfigManager();
  await mng.saveConfig(c);

  props.onClose();
}
</script>

<template>
  <div
    class="flex items-center justify-center fixed left-0 bottom-0 w-full h-full bg-[#00000099]"
  >
    <div class="relative bg-white rounded-lg w-[560px]">
      <div class="relative flex flex-col items-start p-6">
        <!-- Header -->
        <div class="flex items-center w-full mb-10">
          <div class="text-gray-900 font-medium text-2xl font-semibold">
            Settings
          </div>
          <svg
            class="ml-auto fill-current text-gray-700 w-6 h-6 cursor-pointer"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 18 18"
            @click="
              () => {
                onClose();
              }
            "
          >
            <path
              d="M14.53 4.53l-1.06-1.06L9 7.94 4.53 3.47 3.47 4.53 7.94 9l-4.47 4.47 1.06 1.06L9 10.06l4.47 4.47 1.06-1.06L10.06 9z"
            />
          </svg>
        </div>

        <!-- ダウンロード先 -->
        <div class="mb-2 flex items-center">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="w-5 h-5"
            viewBox="0 0 50 50"
            fill="currentColor"
          >
            <path
              d="M7.05 40q-1.2 0-2.1-.925-.9-.925-.9-2.075V11q0-1.15.9-2.075Q5.85 8 7.05 8H19.8q.6 0 1.175.25.575.25.975.65l2.1 2.1h17q1.15 0 2.075.925.925.925.925 2.075v23q0 1.15-.925 2.075Q42.2 40 41.05 40Zm0-29v26h34V14H22.8l-3-3H7.05Zm0 0v26Z"
            />
          </svg>
          <span class="ml-2 text-sm">Download Directory</span>
        </div>
        <div class="relative w-full h-10 mb-10 flex flex-row">
          <div
            :id="dlDirContainerId"
            class="flex bg-[#f0f2f5] items-center px-4 h-10 rounded-md w-full"
          >
            <input
              class="outline-none bg-[#f0f2f5] block w-full font-normal text-sm"
              type="text"
              name=""
              :id="dlDirFieldId"
              placeholder="Download Directory"
              maxlength="240"
              :value="fieldValue"
            />
          </div>
          <div class="ml-2">
            <button
              class="bg-[#ebedf2] hover:bg-[#d8dbe3] w-10 h-10 rounded-md font-semibold tracking-wide cursor-pointer flex items-center justify-center"
              title="Select Folder"
              @click="openFolder"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="w-5 h-5 group-hover:fill-white"
                viewBox="0 0 48 48"
                fill="currentColor"
              >
                <path
                  d="M7 40q-1.15 0-2.075-.925Q4 38.15 4 37V11q0-1.15.925-2.075Q5.85 8 7 8h12.8q.6 0 1.175.25.575.25.975.65l2.1 2.1H41q1.15 0 2.075.925Q44 12.85 44 14H22.75l-3-3H7v26l4.5-17.75q.25-1 1.1-1.625.85-.625 1.85-.625H43.1q1.45 0 2.4 1.15t.55 2.6l-4.4 16.95q-.3 1.2-1.1 1.75T38.5 40Zm3.15-3h28.6l4.2-17h-28.6Zm0 0 4.2-17-4.2 17ZM7 17v-6 6Z"
                />
              </svg>
            </button>
          </div>
        </div>

        <!-- ZIP設定 -->
        <div class="flex flex-row w-full items-center">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="w-5 h-5"
            viewBox="0 0 50 50"
            fill="currentColor"
          >
            <path
              d="M7 40q-1.15 0-2.075-.925Q4 38.15 4 37V11q0-1.15.925-2.075Q5.85 8 7 8h12.75q.6 0 1.175.25.575.25.975.65L24 11h17q1.15 0 2.075.925Q44 12.85 44 14v23q0 1.15-.925 2.075Q42.15 40 41 40Zm25-3h9V14h-9v4.6h4.6v4.6H32v4.6h4.6v4.6H32ZM7 37h20.4v-4.6H32v-4.6h-4.6v-4.6H32v-4.6h-4.6V14h-4.65l-3-3H7v26Zm0-23v-3 26-23Z"
            />
          </svg>
          <span class="ml-2 text-sm">ZIP compression</span>
          <div class="flex-1"></div>
          <ToggleSwitch
            :eleId="'toggle'"
            :defaultValue="zipValue"
            :onChange="onChangeZipValue"
          />
        </div>
        <div class="ml-6">
          <span class="ml-1 text-xs opacity-60"
            >Perfirm ZIP compression when downloading folder.</span
          >
        </div>

        <!-- 余白 -->
        <div class="h-16"></div>

        <!-- 保存ボタン -->
        <div class="flex flex-row w-full">
          <div class="flex-1"></div>
          <button
            class="bg-[#091e33] hover:bg-[#091e33aa] px-4 h-10 rounded-md tracking-wide cursor-pointer text-white flex items-center justify-center"
            @click="saveConfig"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="w-6 h-6 group-hover:fill-white"
              viewBox="0 0 50 50"
              fill="#ffffff"
            >
              <path
                d="M9 42q-1.2 0-2.1-.9Q6 40.2 6 39V9q0-1.2.9-2.1Q7.8 6 9 6h23.9q.6 0 1.175.25.575.25.975.65l6.05 6.05q.4.4.65.975T42 15.1V39q0 1.2-.9 2.1-.9.9-2.1.9Zm30-26.8L32.8 9H9v30h30ZM24 35.75q2.15 0 3.675-1.525T29.2 30.55q0-2.15-1.525-3.675T24 25.35q-2.15 0-3.675 1.525T18.8 30.55q0 2.15 1.525 3.675T24 35.75ZM13.15 18.8h14.9q.65 0 1.075-.425.425-.425.425-1.075v-4.15q0-.65-.425-1.075-.425-.425-1.075-.425h-14.9q-.65 0-1.075.425-.425.425-.425 1.075v4.15q0 .65.425 1.075.425.425 1.075.425ZM9 15.2V39 9Z"
              />
            </svg>

            <span class="ml-2 text-sm font-semibold">Save</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
