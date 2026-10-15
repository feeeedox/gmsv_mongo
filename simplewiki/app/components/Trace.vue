<script setup lang="ts">
import { useRoute } from 'vue-router'
import { ref, watch, onMounted } from 'vue'

const props = defineProps<{
    title?: string
}>()

const route = useRoute()
const separator = "›"

interface HistoryItem {
    name: string
    path: string
}

const previousPages = ref<HistoryItem[]>([])
const activePage = ref("")

const formatSlug = (path: string) => {
    if (path === "/") return "Home"

    const lastSegment = path.split("/").pop() || ""

    return lastSegment
        .replace(/[-_]/g, " ")
        .replace(/\b\w/g, c => c.toUpperCase())
}

function updateHistory() {
    if (!import.meta.client) return

    const currentPath = route.path
    const currentTitle = props.title || formatSlug(currentPath)

    const lastPath = sessionStorage.getItem("wiki_last_path")
    const lastTitle = sessionStorage.getItem("wiki_last_title")

    if (lastPath && lastPath !== currentPath) {
        previousPages.value = previousPages.value.filter(
            p => p.path !== lastPath && p.path !== currentPath
        )

        previousPages.value.push({
            name: lastTitle || formatSlug(lastPath),
            path: lastPath
        })

        if (previousPages.value.length > 4) {
            previousPages.value.shift()
        }

        sessionStorage.setItem(
            "wiki_history",
            JSON.stringify(previousPages.value)
        )
    }

    sessionStorage.setItem("wiki_last_path", currentPath)
    sessionStorage.setItem("wiki_last_title", currentTitle)

    activePage.value = currentTitle
}

onMounted(() => {
    const saved = sessionStorage.getItem("wiki_history")

    if (saved) {
        previousPages.value = JSON.parse(saved)
    }

    updateHistory()
})

watch(
    () => [route.path, props.title],
    () => {
        updateHistory()
    }
)
</script>

<template>
    <div class="bg-header-band border-b border-border">
        <div class="max-w-275 mx-auto py-1 px-5 text-[11.5px] leading-none text-text-muted flex flex-wrap items-center">

            <template v-for="item in previousPages" :key="item.path">
                <NuxtLink
                    :to="item.path"
                    class="text-link hover:underline"
                >
                    {{ item.name }}
                </NuxtLink>

                <span class="text-[#72777d] select-none mx-0.75">
                    {{ separator }}
                </span>
            </template>

            <span class="text-text font-bold">
                {{ activePage }}
            </span>

        </div>
    </div>
</template>