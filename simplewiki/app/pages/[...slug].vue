<script setup lang="ts">
import { useRoute } from 'vue-router'
import Content from '~/components/Content.vue';
import Footer from '~/components/Footer.vue';
import Header from '~/components/Header.vue';
import Sidebar from '~/components/Sidebar.vue';
import ToC from '~/components/ToC.vue';
import Trace from '~/components/Trace.vue';

const route = useRoute()

const { data } = await useAsyncData(`content-${route.path}`, () => {
    return queryCollection('content').path(route.path).first()
})

useSeoMeta({
    title: () => data.value?.title,
    description: () => data.value?.description,
    ogTitle: () => data.value?.title,
    ogDescription: () => data.value?.description,
})

const tocLinks = computed(() => data.value?.body?.toc?.links ?? [])

const isSidebarOpen = ref(false)

defineOgImage('WikiPage', {
  title: data.value?.title,
  description: data.value?.description,
})

</script>

<template>
    <Header @open-sidebar="isSidebarOpen = true" />
    <Trace :title="data?.title" />
    <main class="max-w-275 mx-auto sm:grid sm:grid-cols-[180px_minmax(0,1fr)_240px] w-full items-start min-h-0">
        <Sidebar :is-open="isSidebarOpen" @close="isSidebarOpen = false" />
        <ToC :is-mobile=true :toc-links="tocLinks" />

        <Content :data="data ? data : null" />
        <ToC :is-mobile=false :toc-links="tocLinks" />
    </main>

    <!-- 
    Hi, first of all, thank you for using SimpleWiki! 
    I would love you to keep the footer intact, as it helps support the project and allows me to continue developing it.
    If you want to remove it, please consider supporting the project on GitHub or by donating. You can find the links in the ReadME itself. Thank you for your understanding and support!
    -->
    <Footer/>
</template>