<script setup lang="ts">
import { ref } from 'vue';
import { S3Objects } from '../../repository/s3';

const props = defineProps<{
  s3Objects: S3Objects;
  onClose: Function;
}>();

const containerId = 'create-folder-dialog-container';
const fieldId = 'create-folder-dialog-field';

var fieldborderClass = ref<string>('outline-none');

const onFocus = () => {
  fieldborderClass.value = 'outline-solid border-2 border-black';
};
const onUnFocus = () => {
  fieldborderClass.value = 'outline-none';
};

function close() {
  props.onClose();
}

function run() {
  // input fieldからテキストを取得
  const input = document.getElementById(fieldId) as HTMLInputElement;
  if (!input) {
    return;
  }
  props.s3Objects.createFolder(input.value, () => {
    props.onClose();
    input.value = '';
  });
}
</script>

<template>
  <div
    class="flex items-center justify-center fixed left-0 bottom-0 w-full h-full bg-[#00000099]"
  >
    <div class="bg-white rounded-lg w-96">
      <div class="flex flex-col items-start p-6">
        <div class="flex items-center w-full mb-5">
          <div class="text-gray-900 font-medium text-2xl font-semibold">
            Create Folder
          </div>
          <svg
            class="ml-auto fill-current text-gray-700 w-6 h-6 cursor-pointer"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 18 18"
            @click="close"
          >
            <path
              d="M14.53 4.53l-1.06-1.06L9 7.94 4.53 3.47 3.47 4.53 7.94 9l-4.47 4.47 1.06 1.06L9 10.06l4.47 4.47 1.06-1.06L10.06 9z"
            />
          </svg>
        </div>

        <!-- インプットフィールド -->
        <div :id="containerId" class="w-full mb-8">
          <div
            class="flex bg-[#ebedf2cc] items-center px-4 h-10 rounded-md w-full"
            :class="fieldborderClass"
          >
            <input
              class="bg-[#ebedf2cc] outline-none block w-full font-normal text-base"
              type="text"
              name=""
              :id="fieldId"
              placeholder="Folder Name.."
              maxlength="64"
              @focus="onFocus()"
              @focusout="onUnFocus()"
            />
          </div>
        </div>

        <hr />
        <div class="ml-auto flex">
          <button
            class="bg-transparent hover:bg-gray-500 font-semibold py-2 px-4 border-2 rounded hover:text-white hover:border-transparent text-[#091e33] border-[#091e33]"
            @click="close"
          >
            Cancel
          </button>
          <div class="w-4"></div>
          <button
            class="text-white font-bold py-2 px-4 rounded bg-[#091e33] hover:bg-[#091e33aa]"
            @click="run"
          >
            Create
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
