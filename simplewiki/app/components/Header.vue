<script setup lang="ts">
import { ref, watch } from 'vue'

const { wikiName } = useAppConfig()

const { status, search } = useSearchCollection('content')

const searchQuery = ref('')
const searchResults = ref<any[]>([])
const isDropdownOpen = ref(false)

watch(searchQuery, async (value) => {
    if (!value.trim()) {
        searchResults.value = []
        isDropdownOpen.value = false
        return
    }

    try {
        const results = await search(value, {
            snippet: {
                columns: ['content'],
                around: 40
            }
        })

        searchResults.value = results.slice(0, 6)
        isDropdownOpen.value = searchResults.value.length > 0
    } catch (e) {
        console.error('Error while searching:', e)
    }
})

const closeSearch = () => {
    setTimeout(() => {
        isDropdownOpen.value = false
        searchQuery.value = ''
    }, 150)
}

const { data: navigation } = await useAsyncData('root-redirect', () =>
    queryCollectionNavigation('content')
)

const redirectPath = ref('/')

const firstGroup = navigation.value?.[0]
if (firstGroup) {
    if (firstGroup.children && firstGroup.children.length > 0) {
        redirectPath.value = firstGroup.children?.[0]?.path ?? '/'
    } else if (firstGroup.path) {
        redirectPath.value = firstGroup.path
    }
}

const colorMode = useColorMode()

const setTheme = (theme: 'light' | 'dark') => {
    colorMode.preference = theme
}
</script>

<template>
    <header class="bg-content-bg border-b border-border relative z-50">
        <div
            class="max-w-275 mx-auto pt-2.25 pb-2 px-5 flex flex-col sm:flex-row sm:items-center justify-between sm:gap-6 flex-wrap w-full">
            <div class="flex items-baseline gap-2 justify-center sm:justify-start">
                <NuxtLink :to="redirectPath" class="hover:opacity-90">
                    <h1 class="font-heading text-[1.375rem] font-normal text-text">{{ wikiName }}</h1>
                </NuxtLink>
                <button class="sm:hidden text-text-muted hover:text-text" @click="$emit('open-sidebar')"
                    aria-label="Open navigation">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                        stroke="currentColor" class="w-5 h-5">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                    </svg>
                </button>

                <p class="text-xs text-text-muted italic hidden sm:block">Documentation</p>
            </div>


            <div class="flex flex-row items-center justify-center gap-4">
                <div class="flex flex-col relative">
                    <ClientOnly>
                        <div class="flex gap-1.5 text-[11px] font-body items-end text-center mx-auto sm:mx-0">
                            <span class="text-text-muted hidden sm:block">Design:</span>
                            <button class="cursor-pointer" @click="setTheme('light')"
                                :class="colorMode.value === 'light' ? 'text-focus font-medium' : 'text-link hover:underline'">
                                [ Light ]
                            </button>
                            <button class="cursor-pointer" @click="setTheme('dark')"
                                :class="colorMode.value === 'dark' ? 'text-focus font-medium' : 'text-link hover:underline'">
                                [ Dark ]
                            </button>
                        </div>
                    </ClientOnly>


                    <div class="flex items-stretch relative">
                        <input v-model="searchQuery" @blur="closeSearch"
                            @focus="isDropdownOpen = searchResults.length > 0" :disabled="status !== 'ready'"
                            type="text" :placeholder="status === 'ready' ? 'Search wiki...' : 'Indexing...'"
                            class="w-full sm:w-56 py-1 px-2 border border-border rounded-xs bg-white dark:bg-sidebar-bg font-body text-sm text-text focus:outline-none focus:border-focus focus:ring-1 focus:ring-focus disabled:opacity-50">
                    </div>

                    <div v-if="isDropdownOpen"
                        class="absolute top-full right-0 mt-1 w-72 bg-white dark:bg-sidebar-bg border border-border rounded-xs shadow-lg overflow-hidden max-h-72 overflow-y-auto z-50">
                        <ul class="list-none m-0 p-0">
                            <li v-for="result in searchResults" :key="result.id"
                                class="border-b border-border-light last:border-0">
                                <NuxtLink :to="result.id"
                                    class="block p-2 hover:bg-[#f6f8fa] dark:hover:bg-[#2d333b] text-left no-underline">
                                    <div class="text-xs font-bold text-link truncate">{{ result.title }}</div>

                                    <div v-if="result.snippets?.content"
                                        class="text-[11px] text-text-muted mt-0.5 line-clamp-2 search-snippet"
                                        v-html="result.snippets.content" />
                                </NuxtLink>
                            </li>
                        </ul>
                    </div>

                </div>
            </div>
        </div>
    </header>
</template>

<style>
.search-snippet mark {
    background-color: #fff5b8;
    color: #24292e;
    font-weight: 600;
    padding: 0 2px;
    border-radius: 2px;
}
</style>