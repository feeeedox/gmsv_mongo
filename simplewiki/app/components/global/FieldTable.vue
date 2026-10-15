<script lang="ts" setup>
const props = defineProps<{
    items: Record<string, string>
}>()

function renderValue(value: unknown) {
    const str = value == null
        ? ''
        : typeof value === 'string'
            ? value
            : String(value)

    let html = str
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')

    html = html.replace(/`([^`]+)`/g, '<code>$1</code>')

    html = html.replace(
        /\[([^\]]+)\]\(([^)]+)\)/g,
        '<a href="$2" target="_blank" rel="noopener noreferrer">$1</a>'
    )

    html = html.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')
    html = html.replace(/__([^_]+)__/g, '<strong>$1</strong>')

    html = html.replace(/\*([^*]+)\*/g, '<em>$1</em>')
    html = html.replace(/_([^_]+)_/g, '<em>$1</em>')

    return html
}
</script>

<template>
    <table class="table-auto w-full max-w-140 border-collapse text-13 bg-content-bg">
        <tbody>
            <tr v-for="(value, key) in props.items" :key="key">
                <th class="w-[1%] whitespace-nowrap bg-header-band text-text font-bold text-left px-2.5 py-1.5">
                    {{ key }}
                </th>
                <td 
                    class="w-auto text-text text-left px-2.5 py-1.5 wrap-break-word [&_a]:text-link [&_a]:underline hover:[&_a]:text-link-hover bg-content-bg"
                    v-html="renderValue(value)"
                />
            </tr>
        </tbody>
    </table>
</template>