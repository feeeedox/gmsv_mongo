<script setup lang="ts">
withDefaults(defineProps<{
  icon?: boolean | string
}>(), {
  icon: true
})
</script>

<template>
  <div class="text-13 text-text-muted mb-2.5 relative" :class="icon == true || icon == 'true' ? 'pl-5' : ''">
    <span v-if="icon == true || icon == 'true'" class="absolute left-0 text-text-muted">▸</span>
    <em><slot /></em>
  </div>
</template>
