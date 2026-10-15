<script setup lang="ts">
const props = withDefaults(defineProps<{
  type?: 'beta' | 'new' | 'deprecated' | 'stable' | 'info'
}>(), {
  type: 'info'
})

const styles: Record<string, string> = {
  beta: 'bg-amber-50 text-amber-800 border-amber-300',
  new: 'bg-green-50 text-green-800 border-green-300',
  deprecated: 'bg-red-50 text-red-800 border-red-300',
  stable: 'bg-blue-50 text-blue-800 border-blue-300',
  info: 'bg-gray-100 text-text-muted border-border'
}

const labels: Record<string, string> = {
  beta: 'Beta',
  new: 'New',
  deprecated: 'Deprecated',
  stable: 'Stable',
  info: 'Info'
}
</script>

<template>
  <span
    class="inline-flex items-center text-[11px] font-bold uppercase tracking-wide px-1.5 py-0.5 border rounded-xs align-middle mx-1 opacity-75"
    :class="styles[type]"
  >
    <slot>{{ labels[type] }}</slot>
  </span>
</template>