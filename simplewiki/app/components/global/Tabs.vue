<script setup lang="ts">
import { ref, useSlots, computed } from 'vue'

const slots = useSlots()
const activeTab = ref(0)

const tabItems = computed(() => {
  const defaultSlot = slots.default ? slots.default() : []
  
  const children = defaultSlot.flatMap(node => {
    if (node.type === Symbol.for('v-fgt') && Array.isArray(node.children)) {
      return node.children
    }
    return node
  })

  return children.map((node: any, index) => {
    return {
      title: node.props?.title || `Tab ${index + 1}`,
      vnode: node
    }
  })
})
</script>

<template>
  <div class="max-w-full my-4">
    <ul class="list-none! p-0! m-0! flex! flex-wrap! relative! border-b! border-border-light! z-10!">
      <li 
        v-for="(tab, index) in tabItems" 
        :key="index"
        class="m-[0_0.2em_-1px_0] p-0"
      >
        <button 
          type="button" 
          @click="activeTab = index"
          class="block font-sans text-13 px-3 py-[5.5px] border border-[#a2a9b1] dark:border-border-light outline-none transition-colors duration-100"
          :class="[
            activeTab === index 
              ? 'bg-content-bg border-b-transparent text-[#202122] font-bold pointer-events-none dark:text-[#e5e7eb]' 
              : 'bg-[#f8f9fa] dark:bg-sidebar-bg text-[#3366cc] cursor-pointer hover:bg-content-bg hover:text-[#204a87]'
          ]"
        >
          {{ tab.title }}
        </button>
      </li>
    </ul>

    <div class="bg-content-bg border border-[#a2a9b1] dark:border-border-light border-top-none p-3 text-[14px]">
      <component 
        v-for="(tab, index) in tabItems"
        :key="index"
        :is="tab.vnode"
        v-show="activeTab === index"
      />
    </div>
  </div>
</template>