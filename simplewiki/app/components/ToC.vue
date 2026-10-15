<script setup lang="ts">
defineProps({
    isMobile: {
        type: Boolean,
        required: true,
    },
    tocLinks: {
        type: Array as PropType<any[]>,
        required: true,
    }
})
</script>

<template>
    <div class="sm:ml-4 sm:mt-4 sm:my-0 pt-2 px-3.5 pb-2.5 bg-toc-bg border border-border rounded-xs text-13 [counter-reset:toc-main] sticky top-4 my-8 "
        v-if="tocLinks.length && !isMobile">
        <div class="font-bold mx-0 mt-0 mb-1.5 pb-1.25 border-b border-border-light tracking-[0.01em] text-text!">
            Table of Contents
        </div>

        <ol v-if="tocLinks.length" class="m-0 pl-4 list-decimal list-outside">
            <li v-for="link in tocLinks" :key="link.id"
                class="mt-1 mb-1 [counter-increment:toc-main] [counter-reset:toc-sub]">
                <NuxtLink :to="`#${link.id}`" class="text-text! hover:text-link-hover! no-underline">
                    <span class="text-text-muted! mr-1 [content:counter(toc-main)_'.']"></span>{{ link.text }}
                </NuxtLink>

                <ol v-if="link.children && link.children.length" class="pl-4.5 mt-0.5 mb-1 list-inside list-decimal">
                    <li v-for="subLink in link.children" :key="subLink.id"
                        class="mt-0.75 mb-0.75 [counter-increment:toc-sub]">
                        <NuxtLink :to="`#${subLink.id}`" class="text-text! hover:text-link-hover! no-underline">
                            <span
                                class="text-text-muted! mr-1 [font-variant-numeric:tabular-nums] [content:counter(toc-main)_'.'_counter(toc-sub)]"></span>{{
                                    subLink.text }}
                        </NuxtLink>
                    </li>
                </ol>
            </li>
        </ol>
    </div>
    <div v-if="tocLinks.length && isMobile" class="sm:hidden my-3 px-2">
        <details class="bg-toc-bg border border-border rounded-xs">
            <summary
                class="cursor-pointer select-none px-3 py-2 text-[13px] font-bold text-text flex items-center justify-between list-none">
                <span>Table of Contents</span>
                <span class="text-text-muted text-[11px]">▼</span>
            </summary>

            <div class="px-3 py-2 border-t border-border-light">
                <ol class="m-0 pl-4 list-decimal list-outside [counter-reset:toc-main]">
                    <li v-for="link in tocLinks" :key="link.id"
                        class="mt-1 mb-1 [counter-increment:toc-main] [counter-reset:toc-sub]">

                        <NuxtLink :to="`#${link.id}`" class="text-text hover:text-link-hover no-underline block">
                            {{ link.text }}
                        </NuxtLink>

                        <ol v-if="link.children?.length" class="pl-4 mt-1 list-decimal list-outside">
                            <li v-for="subLink in link.children" :key="subLink.id"
                                class="my-1 [counter-increment:toc-sub]">
                                <NuxtLink :to="`#${subLink.id}`"
                                    class="text-text-muted hover:text-link-hover no-underline block">
                                    {{ subLink.text }}
                                </NuxtLink>
                            </li>
                        </ol>
                    </li>
                </ol>
            </div>
        </details>
    </div>
</template>