<script setup lang="ts">
import { open } from '@tauri-apps/api/dialog';
import { ref, toRefs, watch } from 'vue';
import { S3Objects, S3ProileInfo } from '../repository/s3';
import InputPathField from './atoms/InputPathField.vue';

// Props
const props = defineProps<{
  s3Info: S3ProileInfo;
  s3Objects: S3Objects;
  showCreateDialog: Function;
}>();

const { s3Info, s3Objects } = toRefs(props);

// 選択中のバケット
const selectedBucket = ref<string | null>(null);

// 選択中のパス
const displayPath = ref<string | null>(s3Objects.value.prefix);

// パスのリンク
const links = ref<Array<string>>(s3Objects.value.prefix?.split('/') ?? []);

// 表示したパスの履歴
const histories = ref<Array<string>>([]);

// お気に入り登録されているかを確認
const favorited = ref<boolean>(false);

watch(s3Objects.value, () => {
  // S3バケットの情報が更新された場合に呼ばれる

  // 選択中のバケットと変更があった場合
  if (selectedBucket.value != s3Objects.value.bucket) {
    // パスの指定がある場合
    if (s3Objects.value.prefix) {
      displayPath.value = s3Objects.value.prefix;
      links.value = s3Objects.value.prefix.split('/');
    } else {
      // 指定がない場合は空を設定
      displayPath.value = null;
      links.value = [];
    }

    // 履歴を削除
    histories.value = [];

    // 選択中のバケット更新
    selectedBucket.value = s3Objects.value.bucket;
  }
  // 同一のバケットで変更があった場合
  else if (displayPath.value != s3Objects.value.prefix) {
    // 表示中のパスが存在する場合は履歴に追加
    if (displayPath.value) {
      histories.value.push(displayPath.value);
    }
    // 表示中のパスが存在しないが移動先のパスがある場合(第一階層への移動)
    else if (s3Objects.value.prefix) {
      // 存在しない場合は空を追加
      histories.value.push('');
    }
    console.log(
      'change prefix: ' +
        s3Objects.value.prefix +
        ' -> ' +
        histories.value.length
    );
    // 表示パスを更新
    displayPath.value = s3Objects.value.prefix;
    if (s3Objects.value.prefix) {
      links.value = s3Objects.value.prefix.split('/');
    } else {
      links.value = [];
    }
  }
  // 同一であった場合はリンクのみ更新しておく
  else {
    if (s3Objects.value.prefix) {
      links.value = s3Objects.value.prefix.split('/');
    } else {
      links.value = [];
    }
  }
});

// 表示パスの監視
watch(displayPath, () => {
  // 入力フィールドのテキストを更新
  const ele = document.getElementById(inputFieldId) as HTMLInputElement;
  if (ele) {
    var t = '';
    if (s3Objects.value.bucket) {
      t = s3Objects.value.bucket!;
    }
    if (displayPath.value) {
      t += '/' + displayPath.value;
    }

    ele.value = t;
  }

  // お気に入りかどうかを判定して設定
  favorited.value = s3Info.value.isFavorite(s3Objects.value, displayPath.value);

  // 選択中のパスのディレクトリ、ファイル情報を取得
  if (!s3Objects.value.bucket || displayPath.value == null) {
    return;
  }
  if (displayPath.value != s3Objects.value.prefix) {
    // バケット名を除去した値を通知
    var newpath: string | null = displayPath.value;
    if (newpath[0] == '/') {
      newpath = newpath.replace('/', '');
    } else if (newpath == '') {
      newpath = null;
    }
    s3Objects.value.updatePrefix(newpath);
  }
});

// 表示するパス情報を返却
const displayLinks = () => {
  // 5以下の場合はそのまま返却
  if (links.value.length <= 5) {
    return links.value;
  }
  let offset = links.value.length - 5;
  return links.value.slice(offset);
};

// 前の表示ページに戻る
const onPrevious = () => {
  console.log('histories: ' + histories.value);
  if (histories.value.length == 0) {
    return;
  }
  // 最後のアイテム削除
  let item = histories.value.pop();
  // パスを新たに設定する
  if (item || item == '') {
    displayPath.value = item;
  } else {
    histories.value = [];
    s3Objects.value.updateBucket(null);
  }
};

// 指定のパスに移動
const onmoveByIndex = (index: number) => {
  var list = [];
  var newIndex = index;
  // 表示項目を絞っている場合はその分のインデックスを追加する
  if (links.value.length > 5) {
    newIndex = newIndex + links.value.length - 5;
  }

  // パスからインデックスまでのパスを構築
  if (displayPath.value) {
    let s = displayPath.value.split('/');
    for (const [i, ss] of s.entries()) {
      if (i <= newIndex && ss.length != 0) {
        list.push(ss);
      }
    }

    let newpath = list.join('/');
    // 戻るボタンを押下したときのために前の状態を保持しておく
    histories.value.push(displayPath.value);

    // パスを設定
    displayPath.value = newpath + '/';
  }
};

// 入力フィールドのID
const inputContainerId = 's3_path_container';
const inputFieldId = 's3_path_input';
var visibled = false;
const openMoveField = () => {
  visibled = !visibled;
  const ele = document.getElementById(inputContainerId);
  ele!.style.visibility = visibled ? 'visible' : 'hidden';

  if (visibled) {
    const ele = document.getElementById(inputFieldId) as HTMLInputElement;
    ele!.focus();
  }
};

const inputFieldOnEnter = (val: string) => {
  console.log(val);
  // 戻るボタンを押下したときのために前の状態を保持しておく
  if (displayPath.value) {
    histories.value.push(displayPath.value);
  }

  // バケット名とパス名を分割
  var p = val;
  // スキーマが付与されている場合は除去
  p = p.replace('s3://', '');
  // '/'で区切って一番最初をバケット名として扱う
  const pl = p.split('/');
  if (pl.length == 0) {
    return;
  }
  // バケット名
  const bucket = pl[0];

  // バケットが存在するか確認
  if (!s3Objects.value.checkBucket(bucket)) {
    // 存在しない場合はバケットが存在しない旨のページを表示させる
    s3Objects.value.bucket = null;
    return;
  }

  // パス設定
  const ph = pl.slice(1).join('/');
  s3Objects.value.update(bucket, ph);

  // 入力フィールドを非表示
  openMoveField();
};

// ファイル選択ダイアログを表示
function openDialog() {
  open().then((files) => {
    // ファイルが未選択の場合は特に処理なし
    if (!files) {
      return;
    }

    // 戻り値のタイプ毎に処理する
    if (Array.isArray(files)) {
      for (const p of files) {
        s3Objects.value.putObject(p);
      }
    } else {
      s3Objects.value.putObject(files);
    }
  });
}

// ファイル作成ダイアログを表示
function openCreateDialog() {
  props.showCreateDialog();
}

// お気に入り登録
function onFavorite() {
  s3Info.value.updateFavorite(
    s3Objects.value.bucket!,
    s3Objects.value.prefix ?? '',
    !favorited.value,
    () => {
      favorited.value = !favorited.value;
    }
  );
}
</script>

<template>
  <div class="relative flex items-center pb-6 mt-8">
    <button
      class="bg-[#ebedf2] hover:bg-[#d8dbe3] w-10 h-10 rounded-md font-semibold tracking-wide cursor-pointer flex items-center justify-center"
      :disabled="histories.length == 0"
      title="Back Page"
      @click="onPrevious"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="w-5 h-5 group-hover:fill-white"
        viewBox="0 0 44 44"
        fill="currentColor"
      >
        <path
          d="m22.35 38.95-13.9-13.9q-.25-.25-.35-.5Q8 24.3 8 24q0-.3.1-.55.1-.25.35-.5L22.4 9q.4-.4 1-.4t1.05.45q.45.45.45 1.05 0 .6-.45 1.05L13.1 22.5h24.8q.65 0 1.075.425.425.425.425 1.075 0 .65-.425 1.075-.425.425-1.075.425H13.1l11.4 11.4q.4.4.4 1t-.45 1.05q-.45.45-1.05.45-.6 0-1.05-.45Z"
        />
      </svg>
    </button>

    <button
      class="bg-[#091e33] hover:bg-[#091e33aa] w-10 h-10 rounded-md tracking-wide cursor-pointer text-white ml-3 flex items-center justify-center"
      title="Input Path"
      @click="openMoveField"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="w-5 h-5 group-hover:fill-white"
        viewBox="0 0 46 46"
        fill="#ffffff"
      >
        <path
          d="m26.3 27.5-2.7 2.7q-.45.45-.45 1.05 0 .6.45 1.05.45.45 1.05.45.6 0 1.05-.45l5.25-5.25q.45-.45.45-1.05 0-.6-.45-1.05L25.7 19.7q-.45-.45-1.05-.45-.6 0-1.05.45-.45.45-.45 1.05 0 .6.45 1.05l2.7 2.7h-8.8q-.65 0-1.075.425Q16 25.35 16 26q0 .65.425 1.075.425.425 1.075.425ZM7.05 40q-1.2 0-2.1-.925-.9-.925-.9-2.075V11q0-1.15.9-2.075Q5.85 8 7.05 8H19.8q.6 0 1.175.25.575.25.975.65l2.1 2.1h17q1.15 0 2.075.925.925.925.925 2.075v23q0 1.15-.925 2.075Q42.2 40 41.05 40Zm0-29v26h34V14H22.8l-3-3H7.05Zm0 0v26Z"
        />
      </svg>
    </button>

    <div
      class="relative flex flex-1 items-center text-left h-10 mr-12 box-border"
    >
      <div class="flex flex-wrap items-center">
        <!-- 
            リンクが多い場合は表示しきれないため、ここで数を絞る
            リンク数が5を超える場合は間に [...] を表示する
            -->
        <div class="ml-2 text-base font-semibold" v-if="links.length > 5">
          ...
        </div>

        <!-- アイテムを表示 -->
        <div class="ml-2" v-for="(link, index) in displayLinks()" :key="link">
          <div class="flex" v-if="link && link.length != 0">
            <div class="text-base font-light mr-2" v-if="index != 0">/</div>
            <div
              class="text-base font-semibold underline cursor-pointer"
              :class="{ 'ml-2': index == 0 }"
              @click="onmoveByIndex(index)"
            >
              {{ link }}
            </div>
          </div>
        </div>
      </div>

      <InputPathField
        :containerId="inputContainerId"
        :fieldId="inputFieldId"
        :onEnter="inputFieldOnEnter"
      />
    </div>

    <!-- 検索フィールド -->
    <!-- <div class="flex items-center justify-between">
      <div class="flex bg-[#ebedf2] items-center px-4 h-10 rounded-md">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-4 w-4 text-gray-400"
          viewBox="0 0 20 20"
          fill="currentColor"
        >
          <path
            fill-rule="evenodd"
            d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z"
            clip-rule="evenodd"
          />
        </svg>
        <input
          class="bg-[#ebedf2] outline-none ml-2 block"
          type="text"
          name=""
          id="s"
          placeholder="search..."
        />
      </div>
    </div> -->

    <div class="flex" v-if="s3Objects.bucket">
      <!--
      お気に入りボタン
    -->
      <button
        class="bg-[#ebedf2] hover:bg-[#d8dbe3] w-10 h-10 mr-2 rounded-md font-semibold tracking-wide cursor-pointer flex items-center justify-center"
        title="Favorite"
        @click="onFavorite"
      >
        <svg
          v-if="favorited"
          xmlns="http://www.w3.org/2000/svg"
          class="w-5 h-5 group-hover:fill-white"
          viewBox="0 0 44 44"
          fill="#fa9e0a"
        >
          <path
            d="M15.35 41.2q-.85.7-1.775.05-.925-.65-.575-1.7l3.3-10.75-8.5-6.1q-.9-.65-.575-1.675Q7.55 20 8.65 20H19.2l3.35-11.2q.15-.55.575-.825Q23.55 7.7 24 7.7t.875.275q.425.275.575.825L28.8 20h10.55q1.1 0 1.425 1.025.325 1.025-.575 1.675l-8.5 6.1L35 39.55q.35 1.05-.575 1.7-.925.65-1.775-.05L24 34.6Z"
          />
        </svg>
        <svg
          v-else
          xmlns="http://www.w3.org/2000/svg"
          class="w-5 h-5 group-hover:fill-white"
          viewBox="0 0 44 44"
          fill="currentColor"
        >
          <path
            d="M24 25.15ZM13.5 42.6q-.45.35-.875.025-.425-.325-.275-.875L16.3 28.8 5.9 21.35q-.45-.3-.275-.825Q5.8 20 6.35 20H19.2l4.1-13.6q.1-.25.275-.4.175-.15.425-.15t.425.15q.175.15.275.4L28.8 20h12.85q.55 0 .725.525.175.525-.275.825L31.7 28.8l3.95 12.95q.15.55-.275.875-.425.325-.875-.025l-10.5-8Zm3.65-6.75 6.85-5.2 6.85 5.2-2.75-8.65 6.3-4.1h-7.55L24 14.45l-2.85 8.65H13.6l6.3 4.1Z"
          />
        </svg>
      </button>

      <!--
      フォルダ作成ボタン
    -->
      <button
        class="bg-[#ebedf2] hover:bg-[#d8dbe3] w-10 h-10 mr-2 rounded-md font-semibold tracking-wide cursor-pointer flex items-center justify-center"
        title="Create Folder"
        @click="openCreateDialog"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="w-5 h-5 group-hover:fill-white"
          viewBox="0 0 44 44"
          fill="currentColor"
        >
          <path
            d="M30 32q.65 0 1.075-.425.425-.425.425-1.075v-3h3q.65 0 1.075-.425Q36 26.65 36 26q0-.65-.425-1.075-.425-.425-1.075-.425h-3v-3q0-.65-.425-1.075Q30.65 20 30 20q-.65 0-1.075.425-.425.425-.425 1.075v3h-3q-.65 0-1.075.425Q24 25.35 24 26q0 .65.425 1.075.425.425 1.075.425h3v3q0 .65.425 1.075Q29.35 32 30 32ZM7.05 40q-1.2 0-2.1-.925-.9-.925-.9-2.075V11q0-1.15.9-2.075Q5.85 8 7.05 8H19.8q.6 0 1.175.25.575.25.975.65l2.1 2.1h17q1.15 0 2.075.925.925.925.925 2.075v23q0 1.15-.925 2.075Q42.2 40 41.05 40Zm0-29v26h34V14H22.8l-3-3H7.05Zm0 0v26Z"
          />
        </svg>
      </button>

      <!--
      ファイルアップロードボタン
      tauriのファイル選択ダイアログを表示
    -->
      <button
        class="bg-[#ebedf2] hover:bg-[#d8dbe3] w-10 h-10 rounded-md font-semibold tracking-wide cursor-pointer flex items-center justify-center"
        title="Upload File"
        @click="openDialog"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="w-5 h-5 group-hover:fill-white"
          viewBox="0 0 44 44"
          fill="currentColor"
        >
          <path
            d="M24 32.35q-.65 0-1.075-.425-.425-.425-.425-1.075V13.8l-4.95 4.95q-.45.45-1.05.425-.6-.025-1.05-.475-.45-.45-.45-1.075t.45-1.075l7.5-7.5q.25-.25.5-.35.25-.1.55-.1.3 0 .55.1.25.1.5.35l7.55 7.55q.45.45.45 1.05 0 .6-.45 1.05-.45.45-1.075.45t-1.075-.45l-4.95-4.9v17.05q0 .65-.425 1.075-.425.425-1.075.425ZM11 40q-1.2 0-2.1-.9Q8 38.2 8 37v-5.65q0-.65.425-1.075.425-.425 1.075-.425.65 0 1.075.425Q11 30.7 11 31.35V37h26v-5.65q0-.65.425-1.075.425-.425 1.075-.425.65 0 1.075.425Q40 30.7 40 31.35V37q0 1.2-.9 2.1-.9.9-2.1.9Z"
          />
        </svg>
      </button>
    </div>
  </div>
</template>
