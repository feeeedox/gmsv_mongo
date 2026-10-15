<script setup lang="ts">
defineProps<{
  items: {
    title: string
    url?: string
    note?: string
  }[]
}>()
</script>

<template>
  <div class="mt-8 pt-4 border-t border-border">
    <div class="mb-2 text-[11px] font-bold uppercase tracking-[0.02em] text-text-muted">
      References
    </div>

    <ol class="m-0 pl-5 text-13 space-y-1">
      <li
        v-for="(item, index) in items"
        :key="index"
        :id="`ref-${index + 1}`"
        class="leading-snug"
      >
        <NuxtLink
          v-if="item.url"
          :href="item.url"
          target="_blank"
          rel="noopener"
          class="text-link hover:underline"
        >{{ item.title }}</NuxtLink>
        <span v-else>{{ item.title }}</span>

        <span v-if="item.note" class="text-text-muted"> — {{ item.note }}</span>

        <NuxtLink
          :href="`#cite-${index + 1}`"
          class="ml-1.5 text-text-muted hover:text-link text-[0.7rem]"
          aria-label="Back to citation"
        >↩</NuxtLink>
      </li>
    </ol>
  </div>
</template>
