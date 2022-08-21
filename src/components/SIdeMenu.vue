<script setup lang="ts">
import { withDefaults, ref, toRefs, watch } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { S3ProileInfo, S3Objects, S3Favorite } from '../repository/s3';
import { createResizable } from '../repository/ui';
import { INSPECT_MAX_BYTES } from 'buffer';

const props = withDefaults(
  defineProps<{
    profile: string | null;
    s3Objects: S3Objects;
    s3Info: S3ProileInfo;
    callback: Function;
  }>(),
  {
    // デフォルト値
    callback: () => {}
  }
);

const { profile, s3Objects, s3Info } = toRefs(props);
const favorited = ref<string | null>(null);

watch(profile, () => {
  getBuckets();
});

// バケット情報を取得
function getBuckets() {
  s3Info.value.listBuckets();
}

// 初回の情報を取得
getBuckets();

// バケット選択
function onSelectedBucket(bucket: string) {
  favorited.value = null;
  s3Objects.value.updateBucket(bucket);
}

// お気に入りを選択
function onSelectFavorite(fav: S3Favorite) {
  favorited.value = fav.bucket + fav.prefix;
  s3Objects.value.update(fav.bucket, fav.prefix);
}

function initialize() {
  const item = document.getElementById('sidemenu');
  const resizer = document.createElement('div');
  resizer.classList.add('resizer');
  resizer.style.height = '100vh';
  item!.appendChild(resizer);
  createResizable(item!, resizer);
}

window.onload = () => {
  initialize();
};
</script>

<template>
  <div
    id="sidemenu"
    class="relative min-h-screen max-h-screen w-64 overflow-y-auto bg-[#091e33]"
  >
    <div class="flex flex-col pt-3 pb-6 overflow-hidden">
      <!-- Favorite List -->
      <div class="flex px-3 py-2">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="w-6 h-6 group-hover:fill-white"
          viewBox="0 0 48 48"
          fill="#fa9e0a"
        >
          <path
            d="M15.35 41.2q-.85.7-1.775.05-.925-.65-.575-1.7l3.3-10.75-8.5-6.1q-.9-.65-.575-1.675Q7.55 20 8.65 20H19.2l3.35-11.2q.15-.55.575-.825Q23.55 7.7 24 7.7t.875.275q.425.275.575.825L28.8 20h10.55q1.1 0 1.425 1.025.325 1.025-.575 1.675l-8.5 6.1L35 39.55q.35 1.05-.575 1.7-.925.65-1.775-.05L24 34.6Z"
          />
        </svg>
        <div class="ml-2 text-lg text-white text-left font-bold">Favorites</div>
      </div>
      <ul class="tracking-wide mb-6">
        <li
          class="w-full"
          v-for="fav in s3Info.favorites"
          :key="fav.bucket + fav.prefix"
        >
          <div
            class="relative flex flex-col rounded-md mx-1 px-3 py-3 text-white cursor-pointer hover:bg-white hover:bg-opacity-20 text-left"
            :class="{
              selected_bg: fav.bucket + fav.prefix == favorited
            }"
            @click="onSelectFavorite(fav)"
          >
            <span
              class="font-medium text-gray-100 text-base text-left whitespace-nowrap text-ellipsis overflow-x-hidden"
              :class="{
                selected_text: fav.bucket + fav.prefix == favorited
              }"
              >{{ fav.prefix }}</span
            >
            <span
              class="font-medium text-gray-400 text-sm whitespace-nowrap text-ellipsis overflow-x-hidden"
              :class="{
                selected_text: fav.bucket + fav.prefix == favorited
              }"
              >{{ fav.bucket }}</span
            >
          </div>
        </li>
      </ul>

      <!-- S3 Bucket List -->
      <div class="flex px-3 py-2 mt-4">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="w-6 h-6 group-hover:fill-white"
          viewBox="0 0 48 48"
          fill="#ffffff"
        >
          <path
            d="M16.65 46q-1.15 0-2-.725-.85-.725-1-1.875l-3.4-25.7q-.1-.7.325-1.2Q11 16 11.7 16h24.6q.7 0 1.125.5.425.5.325 1.2l-3.4 25.7q-.15 1.15-1 1.875-.85.725-2 .725Zm0-3H31.4l3.15-24h-21.1l3.2 24ZM24 31.5q2.3 0 3.9-1.6t1.6-3.9v-2q0-.65-.425-1.075Q28.65 22.5 28 22.5q-.65 0-1.075.425Q26.5 23.35 26.5 24v2q0 1.05-.725 1.775-.725.725-1.775.725-1.05 0-1.775-.725Q21.5 27.05 21.5 26v-2q0-.65-.425-1.075Q20.65 22.5 20 22.5q-.65 0-1.075.425Q18.5 23.35 18.5 24v2q0 2.3 1.6 3.9t3.9 1.6ZM30 13q-1 0-1.75-.75t-.75-1.8q0-1 .75-1.725Q29 8 30.05 8q1 0 1.725.725.725.725.725 1.775 0 1-.725 1.75T30 13Zm-10-2q-1.85 0-3.175-1.325Q15.5 8.35 15.5 6.45q0-1.85 1.325-3.15Q18.15 2 20.05 2q1.85 0 3.15 1.3 1.3 1.3 1.3 3.2 0 1.85-1.3 3.175Q21.9 11 20 11Zm11.4 32H16.65 31.4Z"
          />
        </svg>
        <div class="ml-2 text-lg text-white text-left font-bold">Buckets</div>
      </div>

      <ul class="tracking-wide">
        <li class="w-full" v-for="bucket in s3Info.buckets" :key="bucket.name">
          <div
            class="relative flex items-center rounded-md mx-1 px-3 py-3 text-white cursor-pointer hover:bg-white hover:bg-opacity-20"
            :class="{
              selected_bg: bucket.name == s3Objects.bucket && !favorited
            }"
            :title="bucket.name"
            @click="onSelectedBucket(bucket.name)"
          >
            <span
              class="font-medium text-gray-100 text-sm text-left whitespace-nowrap text-ellipsis overflow-x-hidden"
              :class="{
                selected_text: bucket.name == s3Objects.bucket && !favorited
              }"
              >{{ bucket.name }}</span
            >
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
.selected_bg {
  background-color: #ffffff;
}

.selected_text {
  font-weight: 600;
  color: #091e33;
}
</style>
