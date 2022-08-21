<script setup lang="ts">
import { toRefs } from 'vue';
import { S3SelectedObjects } from '../repository/s3';

const props = defineProps<{
  selected: S3SelectedObjects;
}>();

const { selected } = toRefs(props);

function onClose() {
  selected.value.onCloseConfirmDialog();
}

function onAction() {
  selected.value.delete();
}
</script>

<template>
  <div
    class="relative left-0 right-0 bottom-0 h-20 bg-white px-8"
    style="box-shadow: 0px -2px 4px 0px rgba(0, 0, 0, 0.3)"
  >
    <div class="h-full flex items-center content-center">
      <!-- ダウンロード中の情報を表示 -->
      <div class="flex" v-if="selected.downloading">
        <div class="w-6 h-6 flex items-center justify-center">
          <div
            class="w-6 h-6 p-0.5 flex items-center justify-center circle-border"
          >
            <div class="circle-core"></div>
          </div>
        </div>
        <span class="text-[#091e33] ml-2 font-semibold text-sm"
          >Downloading..</span
        >
      </div>

      <!-- ダウンロード完了の情報を表示 -->
      <div class="flex" v-if="selected.downloaded">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="w-5 h-5 group-hover:fill-white"
          viewBox="0 0 46 46"
          fill="#091e33"
        >
          <path
            d="M13.6 42V16.4L24.85 4.5q.7-.7 1.625-.825t1.775.375q.85.5 1.275 1.375.425.875.225 1.825L27.8 16.4h14.95q1.2 0 2.1.9.9.9.9 2.1v4.1q0 .35.075.725t-.075.725l-6.3 14.5q-.45 1.05-1.475 1.8Q36.95 42 35.8 42Zm3-24.35V39h19.85l6.3-14.95V19.4H24.1l2.65-12.45ZM6.95 42q-1.25 0-2.125-.875T3.95 39V19.4q0-1.25.875-2.125T6.95 16.4h6.65v3H6.95V39h6.65v3Zm9.65-3V17.65 39Z"
          />
        </svg>
        <span class="text-[#091e33] ml-2 font-semibold text-sm"
          >Downloaded completed!</span
        >
      </div>

      <!-- 削除ボタン -->
      <button
        class="bg-[#db2d21] hover:bg-[#db2d21aa] px-4 h-10 rounded-md tracking-wide cursor-pointer text-white flex items-center justify-center"
        @click="selected.showConfirmDialog"
        v-if="!selected.downloading && !selected.downloaded"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="w-6 h-6 group-hover:fill-white"
          viewBox="0 0 50 50"
          fill="#ffffff"
        >
          <path
            d="M13.05 42q-1.2 0-2.1-.9-.9-.9-.9-2.1V10.5H9.5q-.65 0-1.075-.425Q8 9.65 8 9q0-.65.425-1.075Q8.85 7.5 9.5 7.5h7.9q0-.65.425-1.075Q18.25 6 18.9 6h10.2q.65 0 1.075.425.425.425.425 1.075h7.9q.65 0 1.075.425Q40 8.35 40 9q0 .65-.425 1.075-.425.425-1.075.425h-.55V39q0 1.2-.9 2.1-.9.9-2.1.9Zm0-31.5V39h21.9V10.5Zm5.3 22.7q0 .65.425 1.075.425.425 1.075.425.65 0 1.075-.425.425-.425.425-1.075V16.25q0-.65-.425-1.075-.425-.425-1.075-.425-.65 0-1.075.425-.425.425-.425 1.075Zm8.3 0q0 .65.425 1.075.425.425 1.075.425.65 0 1.075-.425.425-.425.425-1.075V16.25q0-.65-.425-1.075-.425-.425-1.075-.425-.65 0-1.075.425-.425.425-.425 1.075Zm-13.6-22.7V39 10.5Z"
          />
        </svg>

        <span class="ml-2 text-sm font-semibold">Delete</span>
      </button>

      <div class="flex-1"></div>

      <!-- 選択中のアイテム数を表示 -->
      <div class="flex">
        <span class="text-[#091e33] mr-1 font-semibold text-sm">{{
          selected.checked.length
        }}</span>
        <span class="text-[#091e33] mr-6 opacity-70 text-sm">selected</span>
      </div>

      <!-- ダウンロードボタン -->
      <button
        class="bg-[#091e33] hover:bg-[#091e33aa] px-4 h-10 rounded-md tracking-wide cursor-pointer text-white flex items-center justify-center"
        @click="selected.download()"
        :disabled="selected.downloading || selected.downloaded"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="w-6 h-6 group-hover:fill-white"
          viewBox="0 0 50 50"
          fill="#ffffff"
        >
          <path
            d="M24 31.75q-.3 0-.55-.1-.25-.1-.5-.35l-7.55-7.55q-.45-.45-.425-1.075.025-.625.475-1.075.45-.45 1.075-.45t1.075.45l4.9 4.95V9.5q0-.65.425-1.075Q23.35 8 24 8q.65 0 1.075.425.425.425.425 1.075v17.05l4.95-4.95q.45-.45 1.075-.45t1.075.45q.45.45.45 1.075t-.45 1.075l-7.55 7.55q-.25.25-.5.35-.25.1-.55.1ZM11 40q-1.2 0-2.1-.9Q8 38.2 8 37v-5.65q0-.65.425-1.075.425-.425 1.075-.425.65 0 1.075.425Q11 30.7 11 31.35V37h26v-5.65q0-.65.425-1.075.425-.425 1.075-.425.65 0 1.075.425Q40 30.7 40 31.35V37q0 1.2-.9 2.1-.9.9-2.1.9Z"
          />
        </svg>

        <span class="ml-2 text-sm font-semibold">Download</span>
      </button>
    </div>

    <div v-show="selected.showedDialog">
      <ConfirmDialog
        :title="'Delete objects?'"
        :message="'Deletes the specified object. Is it OK?'"
        :onClose="onClose"
        :onAction="onAction"
        :showing="selected.showedDialog"
        :colorClass="'db2d21'"
      />
    </div>
  </div>
</template>
