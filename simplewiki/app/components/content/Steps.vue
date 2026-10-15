<script setup lang="ts">
import { useSlots, computed } from 'vue'

const slots = useSlots()

const items = computed(() => {
  const defaultSlot = slots.default ? slots.default() : []
  const children = defaultSlot.flatMap(node => {
    if (node.type === Symbol.for('v-fgt') && Array.isArray(node.children)) {
      return node.children
    }
    return node
  })
  return children.map((node: any, index) => ({
    title: node.props?.title || `Step ${index + 1}`,
    vnode: node
  }))
})
</script>

<template>
  <ol class="list-none my-4 mx-0 p-0 max-w-140">
    <li v-for="(item, index) in items" :key="index" class="flex gap-2.5 relative pb-3.5 last:before:hidden before:absolute before:left-2.75 before:top-6 before:bottom-0 before:w-px before:bg-border-light">
      <span class="shrink-0 w-5.75 h-5.75 border border-border rounded-xs bg-header-band color-text text-xs font-bold flex items-center justify-center">{{ index + 1 }}</span>
      <div class="min-w-0 flex-1">
        <div class="font-bold text-[13.5px] text-text mb-0.5">{{ item.title }}</div>
        <div class="text-[13.5px] text-text">
          <component :is="item.vnode" />
        </div>
      </div>
    </li>
  </ol>
</template>