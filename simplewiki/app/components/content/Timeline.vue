<script setup lang="ts">
import { useSlots, computed } from 'vue'

const slots = useSlots()

const events = computed(() => {
  const defaultSlot = slots.default ? slots.default() : []
  const children = defaultSlot.flatMap(node => {
    if (node.type === Symbol.for('v-fgt') && Array.isArray(node.children)) {
      return node.children
    }
    return node
  })
  return children.map((node: any) => ({
    date: node.props?.date || '',
    title: node.props?.title || '',
    vnode: node
  }))
})
</script>

<template>
  <div class="relative pl-5 my-4 border-l-2 border-border-light">
    <div
      v-for="(event, index) in events"
      :key="index"
      class="relative pb-5 last:pb-0"
    >
      <span class="absolute -left-6.25 top-1 w-2.5 h-2.5 rounded-full bg-focus border-2 border-content-bg"></span>
      <div class="text-[11px] font-bold text-text-muted uppercase tracking-wide">
        {{ event.date }}
      </div>
      <div v-if="event.title" class="text-[14px] font-bold text-text mb-0.5">
        {{ event.title }}
      </div>
      <div class="text-[13.5px] text-text">
        <component :is="event.vnode" />
      </div>
    </div>
  </div>
</template>