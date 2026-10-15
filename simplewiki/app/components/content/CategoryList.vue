<script setup lang="ts">
defineProps<{
  items: { label: string, to?: string }[]
}>()
</script>

<template>
  <div class="mt-8 pt-2 border-t border-border-light text-[12.5px] text-text-muted">
    <span class="font-bold mr-1.5">Categories:</span>
    <template v-for="(item, i) in items" :key="item.label">
      <NuxtLink v-if="item.to" :to="item.to" class="text-link">{{ item.label }}</NuxtLink>
      <span v-else class="text-link">{{ item.label }}</span>
      <span v-if="i < items.length - 1" class="mx-1.5 my-0 text-border">|</span>
    </template>
  </div>
</template>