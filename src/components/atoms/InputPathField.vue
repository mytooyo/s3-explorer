<script setup lang="ts">
import { ref, withDefaults } from 'vue';

const props = withDefaults(
  defineProps<{
    containerId: string;
    fieldId: string;
    onEnter: Function;
  }>(),
  {}
);

// パス移動フィールドのクラス名
var moveFieldborderClass = ref<string>('outline-none');

// パス移動フィールドにフォーカスが当たった場合の処理
const moveFieldOnFocus = () => {
  moveFieldborderClass.value = 'outline-solid border-2 border-black';
};
const moveFieldOnUnfocus = () => {
  moveFieldborderClass.value = 'outline-none';
};

// 指定のパスに移動
const inputKeyboardEvent = (event: KeyboardEvent) => {
  if (event.key != 'Enter') {
    return;
  }

  // input fieldからテキストを取得
  const input = document.getElementById(props.fieldId) as HTMLInputElement;
  if (!input) {
    return;
  }

  // 呼び出しもとのコールバックを呼び出す
  props.onEnter(input.value);
};
</script>

<template>
  <div :id="containerId" class="absolute inset-0 ml-2 invisible">
    <div
      class="flex bg-[#ffffff] items-center px-4 h-10 rounded-md w-full"
      :class="[moveFieldborderClass]"
    >
      <input
        class="outline-none bg-[#ffffff] block w-full font-normal text-base"
        type="text"
        name=""
        :id="fieldId"
        placeholder="move to path..."
        maxlength="240"
        @focus="moveFieldOnFocus()"
        @focusout="moveFieldOnUnfocus()"
        @keypress="inputKeyboardEvent"
      />
    </div>
  </div>
</template>
