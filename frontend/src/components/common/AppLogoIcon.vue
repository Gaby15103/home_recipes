<script setup lang="ts">
import type { HTMLAttributes } from 'vue';
import AppLogo from "@/../public/vite.svg";

defineOptions({
  inheritAttrs: false,
});

interface Props {
  className?: HTMLAttributes['class'];
}

defineProps<Props>();
</script>

<template>
  <AppLogo/>

</template>
