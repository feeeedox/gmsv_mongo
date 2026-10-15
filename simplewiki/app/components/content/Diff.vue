<script setup lang="ts">
type DiffLine = {
  type: 'add' | 'remove' | 'context'
  content: string
}

defineProps<{
  lines: DiffLine[]
}>()
</script>

<template>
  <pre class="overflow-x-auto rounded-lg border border-slate-200 bg-slate-50 py-3 font-mono text-sm dark:border-slate-800 dark:bg-[#0d1117]"><code class="block min-w-full"><span
    v-for="(line, i) in lines"
    :key="i"
    class="block px-4 whitespace-pre-wrap transition-colors duration-150"
    :class="{
      'bg-green-100 text-green-800 dark:bg-[#2ea44f]/15 dark:text-[#3fb950]': line.type === 'add',
      'bg-red-100 text-red-800 dark:bg-[#f85149]/15 dark:text-[#f85149]': line.type === 'remove',
      'text-slate-700 dark:text-[#c9d1d9]': line.type === 'context'
    }"
  ><span 
    v-if="line.type === 'add' || line.type === 'remove'" 
    class="inline-block w-4 select-none opacity-60"
   >{{ line.type === 'add' ? '+' : '-' }}</span><span v-else class="inline-block w-4 select-none"> </span>{{ line.content }}</span></code></pre>
</template>