<script setup lang="ts">
import { ref, toRefs, watch, withDefaults } from 'vue';

const props = withDefaults(
  defineProps<{
    title: string;
    message: string;
    leftbutton?: string;
    rightButton?: string;
    colorClass?: string;
    showing: boolean;
    onClose: Function;
    onAction: Function;
  }>(),
  {
    leftbutton: 'Cancel',
    rightButton: 'OK',
    colorClass: '#091e33'
  }
);

// 実行中を示すフラグ
const running = ref<boolean>(false);

// 表示中を示すフラグ
const { showing } = toRefs(props);

// 表示状態を監視
watch(showing, () => {
  // 表示状態が変わったら実行中のフラグを変更
  running.value = false;
});

const close = () => {
  if (running.value) {
    return;
  }
  props.onClose();
};

const run = () => {
  running.value = true;
  props.onAction();
};

// クラス名定義
const leftTextColor = 'text-[#' + props.colorClass + ']';
const leftBorderColor = 'border-[#' + props.colorClass + ']';
</script>

<template>
  <div
    class="flex items-center justify-center fixed left-0 bottom-0 w-full h-full bg-[#00000099]"
  >
    <div class="bg-white rounded-lg w-1/2">
      <div class="flex flex-col items-start p-6">
        <div class="flex items-center w-full">
          <div class="text-gray-900 font-medium text-lg font-semibold">
            {{ title }}
          </div>
          <svg
            class="ml-auto fill-current text-gray-700 w-6 h-6 cursor-pointer"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 18 18"
            :disabled="running"
            @click="close"
          >
            <path
              d="M14.53 4.53l-1.06-1.06L9 7.94 4.53 3.47 3.47 4.53 7.94 9l-4.47 4.47 1.06 1.06L9 10.06l4.47 4.47 1.06-1.06L10.06 9z"
            />
          </svg>
        </div>
        <hr />
        <div class="mt-4 mb-6 text-left text-base">
          {{ message }}
        </div>
        <hr />
        <div class="ml-auto flex">
          <button
            class="bg-transparent hover:bg-gray-500 font-semibold py-2 px-4 border-2 rounded hover:text-white hover:border-transparent"
            :class="[leftBorderColor, leftTextColor]"
            :disabled="running"
            @click="close"
          >
            {{ leftbutton }}
          </button>
          <div class="w-4"></div>
          <button
            class="text-white font-bold py-2 px-4 rounded"
            :class="[
              'bg-[#' + colorClass + ']',
              'hover:bg-[#' + colorClass + 'aa]'
            ]"
            :disabled="running"
            @click="run"
          >
            {{ rightButton }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
