<script setup lang="ts">
defineProps<{
  items: { title: string, url: string, note?: string }[]
}>()
</script>

<template>
  <ul class="mt-1.5 mb-4 pl-5 text-13">
    <li v-for="item in items" :key="item.url" class="mx-0.75 list-disc">
      <a :href="item.url" target="_blank" rel="noopener">{{ item.title }}</a>
      <span v-if="item.note" class="text-text-muted"> — {{ item.note }}</span>
    </li>
  </ul>
</template>
