<script setup lang="ts">
defineProps<{
  keys: string
}>()
</script>

<template>
  <span class="inline-flex items-center gap-0.5">
    <kbd v-for="(key, i) in keys.split('+')" :key="i" class="bg-header-band border border-border border-b-2 rounded-xs py-px px-1.25 text-text text-[0.7188rem] font-mono">{{ key }}</kbd>
  </span>
</template> 