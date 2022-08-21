<script setup lang="ts">
import { toRefs, watch } from 'vue';
import {
  S3ObjectInterface,
  S3Objects,
  S3SelectedObjects
} from '../repository/s3';
import ObjectNotFound from './empty/ObjectNotFound.vue';

const props = defineProps<{
  s3Objects: S3Objects;
  selected: S3SelectedObjects;
}>();

const { s3Objects, selected } = toRefs(props);

// S3オブジェクトを監視
watch(s3Objects.value, () => {
  listObjects();
});

// バケット情報を取得
function listObjects() {
  s3Objects.value.listObjects((res: Array<S3ObjectInterface>) => {
    // 選択中の情報は一旦空にする
    selected.value.checked = [];
  });
}
// 初回の情報を取得
listObjects();
</script>

<template>
  <div class="relative">
    <fieldset class="px-8">
      <div class="-mx-4 sm:-mx-8 px-4 sm:px-8 pt-0 pb-12 overflow-x-auto">
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
                  class="px-4 py-3 w-24 border-b-2 border-gray-200 bg-[#ebedf2] text-left text-xs font-semibold text-gray-600 uppercase tracking-wider"
                >
                  Size
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
                v-for="(item, index) in s3Objects.list"
                :key="item.key"
                class="h-12 cursor-pointer hover:bg-sky-700 hover:bg-opacity-20"
              >
                <td
                  class="px-4 py-2 border-b border-gray-100 bg-white text-sm text-left"
                >
                  <div class="flex items-center">
                    <input
                      type="checkbox"
                      :id="'s3-obj' + index"
                      :checked="selected.checked.includes(index)"
                      class="mr-4"
                      @change="selected.onCheckBox(index)"
                    />
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      class="w-4 h-4 group-hover:fill-white mr-3"
                      viewBox="0 0 46 46"
                      fill="#eb8315"
                      v-if="item.is_folder"
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
                    <div class="flex-1" @click="s3Objects.onSelected(item)">
                      <p class="text-gray-900 whitespace-no-wrap">
                        {{ item.key }}
                      </p>
                    </div>
                  </div>
                </td>
                <td
                  class="px-4 py-2 border-b border-gray-100 bg-white text-sm text-right"
                >
                  <p class="text-gray-500 whitespace-no-wrap">
                    {{ item.is_folder ? '-' : item.size }}
                  </p>
                </td>
                <td
                  class="px-4 py-2 border-b border-gray-100 bg-white text-sm text-left"
                >
                  <p class="text-gray-500 whitespace-no-wrap">
                    {{ item.last_modified }}
                  </p>
                </td>
              </tr>
            </tbody>
          </table>
          <div v-if="s3Objects.list.length == 0">
            <ObjectNotFound />
          </div>
        </div>
      </div>
    </fieldset>
  </div>
</template>

<style scoped>
.circle-border {
  border-radius: 50%;
  background: #091e33;
  background: linear-gradient(0deg, rgba(63, 249, 220, 0.1) 33%, #091e33 100%);
  animation: spin 0.9s linear 0s infinite;
}

.circle-core {
  width: 100%;
  height: 100%;
  background-color: white;
  border-radius: 50%;
}

@keyframes spin {
  from {
    transform: rotate(0);
  }
  to {
    transform: rotate(359deg);
  }
}
</style>
