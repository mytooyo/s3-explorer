<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, watch } from "vue";
import LocalExplorerMenu from "../components/LocalExplorerMenu.vue";

// // propsから監視したい値を`toRefs`を用いることでリアクティブに取得する
// const { path } = toRefs(props);

// 選択中のパス
const path = ref<string>("/Users/mytooyo");

// パス内のディレクトリ、ファイルデータ
var list = ref<Array<GetFileInterface>>([]);

const onChangePath = (name: string) => {
  path.value = name;
};

// 変更監視を行う
watch(path, () => {
  // 選択中のパスのディレクトリ、ファイル情報を取得
  getFiles();
});

function getFiles() {
  // 選択中のパスのディレクトリ、ファイル情報を取得
  invoke<Array<GetFileInterface>>("get_files", { dirName: path.value })
    .then((res) => {
      // データを更新
      list.value = res;

      // 現在のパスを入力フィールドに設定しておく
      const input = document.getElementById(
        "move_path_input"
      ) as HTMLInputElement;
      if (!input) {
        return;
      }
      input.value = path.value;
    })
    .catch((e) => console.error(e));
}
// 初回の情報を取得
getFiles();

// 初期表示用にホームディレクトリを取得
function getInitialDir() {
  invoke<string | null>("get_home_dir").then((p) => {
    if (p) {
      path.value = p;
    }
  });
}
getInitialDir();

// クリック時の動作
const onclick = (item: GetFileInterface) => {
  // ファイルの場合は無視
  if (!item.is_dir) {
    return;
  }

  // 現在のディレクトリに追加して新たなパスとして設定
  if (path.value.slice(-1) == "/") {
    path.value = path.value + item.name;
  } else {
    path.value = path.value + "/" + item.name;
  }
};

// Rust側とのインタフェース
interface GetFileInterface {
  name: string;
  file_size: number;
  group: string;
  user: string;
  permission: string;
  update_at: string;
  is_dir: boolean;
}
</script>

<template>
  <div
    class="relative bg-[#f2f5f9] p-8 rounded-md w-full min-h-screen overflow-y-auto overflow-x-hidden box-border"
    style="-webkit-box-sizing: border-box"
  >
    <LocalExplorerMenu :onChangePath="onChangePath" :path="path" />
    <div>
      <div class="-mx-4 sm:-mx-8 px-4 sm:px-8 py-0 overflow-x-auto">
        <div class="inline-block min-w-full rounded-lg overflow-hidden">
          <table class="file_table min-w-full leading-normal">
            <thead>
              <tr>
                <th
                  class="px-4 py-3 border-b-2 border-gray-200 bg-[#ebedf2] text-left text-xs font-semibold text-gray-600 uppercase tracking-wider"
                >
                  Name
                </th>
                <th
                  class="px-4 py-3 w-36 border-b-2 border-gray-200 bg-[#ebedf2] text-left text-xs font-semibold text-gray-600 uppercase tracking-wider"
                >
                  Change at
                </th>
                <th
                  class="px-4 py-3 w-24 border-b-2 border-gray-200 bg-[#ebedf2] text-left text-xs font-semibold text-gray-600 uppercase tracking-wider"
                >
                  Size
                </th>
                <th
                  class="px-4 py-3 w-40 border-b-2 border-gray-200 bg-[#ebedf2] text-left text-xs font-semibold text-gray-600 uppercase tracking-wider"
                >
                  Kind
                </th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="item in list"
                :key="item.name"
                class="cursor-pointer hover:bg-sky-700 hover:bg-opacity-20"
                @click="onclick(item)"
              >
                <td class="px-4 py-2 border-b border-gray-100 bg-white text-sm">
                  <div class="flex items-center">
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      class="w-4 h-4 group-hover:fill-white mr-3"
                      viewBox="0 0 46 46"
                      fill="#eb8315"
                      v-if="item.is_dir"
                    >
                      <path
                        d="M7.05 40q-1.2 0-2.1-.925-.9-.925-.9-2.075V11q0-1.15.9-2.075Q5.85 8 7.05 8H19.8q.6 0 1.175.25.575.25.975.65l2.1 2.1h17q1.15 0 2.075.925.925.925.925 2.075v23q0 1.15-.925 2.075Q42.2 40 41.05 40Zm0-29v26h34V14H22.8l-3-3H7.05Zm0 0v26Z"
                      />
                    </svg>
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      class="w-4 h-4 group-hover:fill-white mr-3"
                      viewBox="0 0 46 46"
                      fill="#1595eb"
                      v-else
                    >
                      <path
                        d="M11 44q-1.2 0-2.1-.9Q8 42.2 8 41V7q0-1.2.9-2.1Q9.8 4 11 4h16.8q.6 0 1.175.25.575.25.975.65l9.15 9.15q.4.4.65.975T40 16.2V41q0 1.2-.9 2.1-.9.9-2.1.9Zm16.55-29.2V7H11v34h26V16.3h-7.95q-.65 0-1.075-.425-.425-.425-.425-1.075ZM11 7v9.3V7v34V7Z"
                      />
                    </svg>
                    <p class="text-gray-900 whitespace-no-wrap">
                      {{ item.name }}
                    </p>
                  </div>
                </td>
                <td
                  class="px-4 py-2 border-b border-gray-100 bg-white text-sm text-left"
                >
                  <p class="text-gray-500 whitespace-no-wrap">
                    {{ item.update_at }}
                  </p>
                </td>
                <td
                  class="px-4 py-2 border-b border-gray-100 bg-white text-sm text-right"
                >
                  <p
                    class="text-gray-500 whitespace-no-wrap"
                    v-if="!item.is_dir"
                  >
                    {{ item.file_size }}
                  </p>
                </td>

                <td
                  class="px-4 py-2 border-b border-gray-100 bg-white text-xs text-left"
                >
                  <span
                    class="relative inline-block px-3 py-1 font-semibold text-green-800 leading-tight"
                  >
                    <span
                      aria-hidden
                      class="absolute inset-0 bg-green-200 opacity-50 rounded-full"
                    ></span>
                    <span class="relative">{{ item.permission }}</span>
                  </span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>
