<script setup lang="ts">
const route = useRoute()

const props = defineProps<{
  id?: string
}>()
const href = computed(() => `${route.path}#${props.id}`)
</script>

<template>
  <h2 :id="id" >
    <NuxtLink v-if="id" :to="href">
      <slot />
    </NuxtLink>
    <slot v-else />
  </h2>
</template>
