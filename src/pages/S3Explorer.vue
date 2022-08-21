<script setup lang="ts">
import { ref, toRefs } from 'vue';
import CreateFolderDialog from '../components/dialog/CreateFolderDialog.vue';
import AccessDenied from '../components/empty/AccessDenied.vue';
import NoSelectedBucket from '../components/empty/NoSelectedBucket.vue';
import S3ExplorerMenu from '../components/S3ExplorerMenu.vue';
import S3ExplorerObjects from '../components/S3ExplorerObjects.vue';
import { S3Objects, S3ProileInfo, S3SelectedObjects } from '../repository/s3';

const props = defineProps<{
  s3Info: S3ProileInfo;
  s3Objects: S3Objects;
  selected: S3SelectedObjects;
}>();

// 選択中のバケット名
const { s3Info, s3Objects, selected } = toRefs(props);

// フォルダ作成ダイアログフラグ
const showCreateDialog = ref<boolean>(false);

function onCloseCreateDialog() {
  showCreateDialog.value = false;
}

function onShowCreateDialog() {
  showCreateDialog.value = true;
}
</script>

<template>
  <div
    class="relative bg-[#f2f5f9] w-full min-h-screen overflow-y-auto overflow-x-hidden box-border"
    style="-webkit-box-sizing: border-box"
  >
    <S3ExplorerMenu
      class="relative px-8"
      :s3Info="s3Info"
      :s3Objects="s3Objects"
      :showCreateDialog="onShowCreateDialog"
    />
    <S3ExplorerObjects
      v-if="s3Objects.bucket && !s3Info.errored"
      class="relative"
      :s3Objects="s3Objects"
      :selected="selected"
    />
    <div v-if="!s3Objects.bucket && !s3Info.errored">
      <NoSelectedBucket />
    </div>
    <AccessDenied v-if="s3Info.errored" />

    <div v-show="showCreateDialog">
      <CreateFolderDialog
        :s3Objects="s3Objects"
        :onClose="onCloseCreateDialog"
      />
    </div>
  </div>
</template>
