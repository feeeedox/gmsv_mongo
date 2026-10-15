<script setup lang="ts">
defineProps<{
  to: string
  title: string
  description?: string
}>()
</script>

<template>
  <NuxtLink
    :to="to"
    class="my-3.5 flex max-w-120 items-start gap-2.5 border border-border bg-content-bg px-3 py-2.5 no-underline! hover:bg-header-band hover:no-underline!"
  >
    <div class="text-[15px] font-bold leading-[1.3] text-link">
      →
    </div>

    <div>
      <div class="text-[13.5px] font-bold text-link visited:text-link-visited">
        {{ title }}
      </div>

      <div
        v-if="description"
        class="mt-0.5 text-[12.5px] text-text-muted"
      >
        {{ description }}
      </div>
    </div>
  </NuxtLink>
</template>