<script setup lang="ts">
import { ref, toRefs, watch } from 'vue';

const props = defineProps<{
  eleId: string;
  defaultValue: boolean;
  onChange: Function;
}>();

const { defaultValue } = toRefs(props);

// 値
const val = ref<boolean>(defaultValue.value);

watch(defaultValue, () => {
  val.value = defaultValue.value;
});

function changed(e: Event) {
  const { target } = e;
  if (target instanceof HTMLInputElement) {
    props.onChange(target.checked);
  }
}
</script>

<template>
  <div class="toggle-switch">
    <input
      id="toggle"
      class="toggle-input"
      type="checkbox"
      :checked="val"
      @change="changed"
    />
    <label for="toggle" class="toggle-label" />
  </div>
</template>

<style scoped lang="scss">
$width: 44px;
$height: 24.64px;

input {
  position: absolute;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  z-index: 5;
  opacity: 0;
  cursor: pointer;
}

label {
  width: $width;
  height: $height;
  background: #ccc;
  position: relative;
  display: inline-block;
  border-radius: 46px;
  transition: 0.4s;
  box-sizing: border-box;
  &:after {
    content: '';
    position: absolute;
    width: $height;
    height: $height;
    border-radius: 100%;
    left: 0;
    top: 0;
    z-index: 2;
    background: #fff;
    box-shadow: 0 0 5px rgba(0, 0, 0, 0.2);
    transition: 0.3s;
  }
}

input:checked {
  + label {
    // background-color: #4bd865;
    background-color: #091e33;
    &:after {
      left: 20px;
    }
  }
}

p {
  margin-top: 50px;
  text-align: center;
  font-weight: bold;
}

.toggle-switch {
  position: relative;
  width: $width;
  height: $height;
  margin: auto;
}
</style>
