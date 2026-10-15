<script setup lang="ts">
withDefaults(defineProps<{
  title?: string
  open?: string | boolean
}>(), {
  title: 'Show Details',
  open: false
})
</script>

<template>
  <details
    class="border border-border rounded-xs my-4 bg-content-bg"
    :open="open === true || open === 'true'"
  >
    <summary
      class="cursor-pointer select-none px-3 py-1.5 bg-header-band border-b border-border text-[13px] font-bold text-text list-none flex items-center justify-between"
    >
      {{ title }}
      <span class="text-text-muted text-[11px]">▼</span>
    </summary>
    <div class="px-3 py-2 text-[13.5px]">
      <slot />
    </div>
  </details>
</template>

<style scoped>
summary::-webkit-details-marker {
  display: none;
}
details[open] summary span {
  transform: rotate(180deg);
  display: inline-block;
}
</style>