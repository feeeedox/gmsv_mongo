<script setup lang="ts">
defineProps({
    data: {
        type: [Object, null],
        required: true,
    }
})
</script>

<template>
    <div class="bg-content-bg pt-4.5 px-6.5 pb-10 border-r border-l border-border min-h-screen min-w-0">
        <ContentRenderer v-if="data" :value="data" class="renderer" :components="{
            h1: 'ProseH1',
            h2: 'ProseH2',
            h3: 'ProseH3',
            h4: 'ProseH4',
            pre: 'ProsePre',
            table: 'ProseTable',
            FieldTable: 'FieldTable',
            Badge: 'Badge',
            Accordion: 'Accordion',
            Timeline: 'Timeline',
            Collapsible: 'Collapsible',
            Diff: 'Diff',
            Steps: 'Steps',
            Keyboard: 'Keyboard',
            Hatnote: 'Hatnote',
            PullQuote: 'PullQuote',
            CategoryList: 'CategoryList',
            ReferenceList: 'ReferenceList',
            SeeAlso: 'SeeAlso',
            LinkCard: 'LinkCard',
        }" />
        <p v-else class="text-red-900">No content available.</p>
    </div>
</template>

<style scoped lang="scss">
.renderer :deep([align="left"]) {
    text-align: left;
}

.renderer :deep([align="center"]) {
    text-align: center;
}

.renderer :deep([align="right"]) {
    text-align: right;
}

.renderer :deep(h1) {
    font-family: var(--font-heading);
    font-size: 24px;
    font-weight: normal;
    margin: 0 0 4px;
    padding-bottom: 6px;
    border-bottom: 1px solid var(--border-light);
}

.renderer :deep(h2) {
    font-family: var(--font-heading);
    font-size: 20px;
    font-weight: normal;
    margin: 20px 0 8px;
    padding-bottom: 4px;
    border-bottom: 1px solid var(--border-light);
}

.renderer :deep(h2 > a) {
    color: var(--text);
}

.renderer :deep(h3) {
    font-size: 15px;
    font-weight: bold;
    margin: 14px 0 6px;
}

.renderer :deep(h3 > a) {
    color: var(--text);
}

.renderer :deep(h4) {
    font-size: 13px;
    font-weight: bold;
    margin: 12px 0 6px;
}

.renderer :deep(h4 > a) {
    color: var(--text);
}

.renderer :deep(br) {
    content: "";
    display: block;
    margin: 8px 0;
}
    
.renderer :deep(h5) {
    font-size: 12px;
    font-weight: bold;
    margin: 10px 0 4px;
}

.renderer :deep(h5 > a) {
    color: var(--text);
}

.renderer :deep(ul) {
    list-style-type: disc;
    margin: 6px 0 12px;
    padding-left: 28px;
}

.renderer :deep(li) {
    margin: 1px 0;
}

.renderer :deep(ol) {
    list-style-type: decimal;
    margin: 6px 0 12px;
    padding-left: 28px;
}

.renderer :deep(pre) {
    background-color: #f6f8fa !important;
    border: 1px solid var(--border-light, #e1e4e8);
    border-radius: 6px;
    padding: 16px;
    margin: 16px 0;
    display: block !important;
    width: 100% !important;
    max-width: 100% !important;
    overflow-x: auto;
    white-space: pre;
    word-break: normal;
    font-family: ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, Liberation Mono, monospace;
    font-size: 13.5px;
    line-height: 1.5;

    .dark & {
        background-color: #0d1117 !important;
        border-color: var(--border-dark, #30363d);
    }
}


.renderer :deep(pre code) {
    background: none !important;
    padding: 0 !important;
    border: none !important;
    font-family: inherit;
    color: inherit;
}

.renderer :deep(.highlight) {
    background-color: #fffbdd;
    display: block;
    margin: 0 -16px;
    padding: 0 16px;
    border-left: 3px solid #ffd33d;

    .dark & {
        background-color: #1a1c1e;
        border-color: #ffd33d;
    }
}

.renderer :deep(hr) {
    border: none;
    border-top: 1px solid var(--border-light);
    margin: 20px 0;
}

.renderer :deep(blockquote) {
    border-left: 4px solid var(--border-light);
    padding-left: 12px;
    margin: 12px 0;
    color: var(--text-muted);
}

.renderer :deep(table) {
    border-collapse: collapse;
    width: 100%;
    max-width: 560px;
    margin: 6px 0 16px;
    font-size: 13px;
    background: var(--content-bg);
}

.renderer :deep(th),
.renderer :deep(td) {
    border: 1px solid var(--border);
    padding: 5px 10px;
    text-align: left;
}

.renderer :deep(th) {
    background: var(--header-band);
    font-weight: bold;
}

.renderer :deep(tr:hover td) {
    background: var(--row-hover-bg);
}

.renderer :deep(img) {
    max-width: 100%;
    height: auto;
}
</style>
