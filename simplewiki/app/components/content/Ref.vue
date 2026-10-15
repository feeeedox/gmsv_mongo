<script setup lang="ts">
const props = defineProps<{
  n: number | string
}>()

const route = useRoute()

const href = computed(() => {
  const hash = `#ref-${props.n}`
  return route.path + hash
})
</script>

<template>
  <sup class="inline-block leading-none">
    <NuxtLink
      :href="href"
      :id="`cite-${props.n}`"
      class="text-link hover:underline text-[0.95em] font-normal px-0.5"
      :aria-label="`Reference ${props.n}`"
    >[{{ props.n }}]</NuxtLink>
  </sup>
</template>
