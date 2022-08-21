<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
import { S3Objects } from "../repository/s3";

// 親からのパラメータ
const props = defineProps<{
  s3Objects: S3Objects;
}>();

// 取得したリスト
const list = ref<Array<S3BucketInterface>>([]);

// バケット情報を取得
function getBuckets() {
  invoke<Array<S3BucketInterface>>("list_buckets").then((res) => {
    list.value = res;
  });
}

// 初回の情報を取得
getBuckets();

// Rust側とのインタフェース
interface S3BucketInterface {
  name: string;
  created_at: string;
  location: string;
}
</script>

<template>
  <div class="relative -mx-4 px-12 py-0 overflow-x-auto">
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
              class="px-4 py-3 w-32 border-b-2 border-gray-200 bg-[#ebedf2] text-left text-xs font-semibold text-gray-600 uppercase tracking-wider"
            >
              Location
            </th>
            <th
              class="px-4 py-3 w-44 border-b-2 border-gray-200 bg-[#ebedf2] text-left text-xs font-semibold text-gray-600 uppercase tracking-wider"
            >
              Change at
            </th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="item in list"
            :key="item.name"
            class="h-12 cursor-pointer hover:bg-sky-700 hover:bg-opacity-20"
            @click="s3Objects.updateBucket(item.name)"
          >
            <td
              class="px-4 py-2 border-b border-gray-100 bg-white text-sm text-left"
            >
              <p class="text-gray-900 whitespace-no-wrap">
                {{ item.name }}
              </p>
            </td>
            <td
              class="px-4 py-2 border-b border-gray-100 bg-white text-sm text-left"
            >
              <p class="text-gray-500 whitespace-no-wrap">
                {{ item.location }}
              </p>
            </td>
            <td
              class="px-4 py-2 border-b border-gray-100 bg-white text-sm text-left"
            >
              <p class="text-gray-500 whitespace-no-wrap">
                {{ item.created_at }}
              </p>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style scoped>
.file_table tr:hover td {
  background-color: #e1e4eb5b;
}
</style>
