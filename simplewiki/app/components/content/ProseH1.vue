<script setup lang="ts">
const route = useRoute()

const props = defineProps<{
  id?: string
}>()
const href = computed(() => `${route.path}#${props.id}`)
</script>

<template>
  <h1 :id="id" >
    <NuxtLink v-if="id" :to="href" class="text-text!">
      <slot />
    </NuxtLink>
    <slot v-else />
  </h1>
</template>
