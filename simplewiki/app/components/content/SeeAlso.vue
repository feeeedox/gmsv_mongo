<script setup lang="ts">
defineProps<{
  items: { title: string, to: string, description?: string, external?: boolean }[]
}>()
</script>

<template>
  <div class="my-4 border border-border-light bg-sidebar-bg px-3 py-2.5">
    <div class="mb-1 text-[11px] font-bold uppercase tracking-[0.02em] text-text-muted">
      See Also
    </div>

    <ul class="m-0 list-disc pl-4.5 text-13">
      <li
        v-for="item in items"
        :key="item.to"
        class="my-0.5"
      >
        <NuxtLink
          :to="item.to"
          :target="item.external ? '_blank' : undefined"
          class="font-bold text-link"
        >
          {{ item.title }}
        </NuxtLink>

        <span
          v-if="item.description"
          class="text-text-muted"
        >
          : {{ item.description }}
        </span>
      </li>
    </ul>
  </div>
</template>