<template>
    <div :class="['info-box', type]" :style="{ width: width }">
        <div class="info-box-icon">
            <img :src="withBase(`/icons/${images[type]}`, baseURL)" alt="" />
        </div>
        <div class="info-box-content">
            <slot />
        </div>
    </div>
</template>

<script setup>
import { withBase } from 'ufo'

const baseURL = useRuntimeConfig().app.baseURL

defineProps({
    type: {
        type: String,
        default: 'info'
    },
    width: {
        type: String,
        default: '100%' 
    }
})

const images = {
    alert: 'alert.png',
    download: 'download.png',
    help: 'help.png',
    important: 'important.png',
    info: 'info.png',
    tip: 'tip.png',
    todo: 'todo.png',
}
</script>

<style scoped lang="scss">
.info-box {
    display: flex;
    gap: 1rem;
    padding: 1rem;
    margin: 1.5rem 0;
    max-width: 100%;
}

.info-box.alert {
    border-left: 4px solid #f56565;
    background-color: #fff5f5;

    .dark & {
        background-color: #4c1d1d;
    }
}

.info-box.download {
    border-left: 4px solid #4fa808;
    background-color: #f0fff0;

    .dark & {
        background-color: #1a2e1a;
    }
}

.info-box.help {
    border-left: 4px solid #4299e1;
    background-color: #ebf8ff;

    .dark & {
        background-color: #1a2e3a;
    }
}

.info-box.important {
    border-left: 4px solid #ed8936;
    background-color: #fffaf0;

    .dark & {
        background-color: #3a2e1a;
    }
}

.info-box.info {
    border-left: 4px solid #4299e1;
    background-color: #ebf8ff;

    .dark & {
        background-color: #1a2e3a;
    }
}

.info-box.tip {
    border-left: 4px solid #fdf998;
    background-color: #fefce8;

    .dark & {
        background-color: #3a3a1a;
    }
}

.info-box.todo {
    border-left: 4px solid #f6ad55;
    background-color: #fffaf0;
    
    .dark & {
        background-color: #3a2e1a;
    }
}

.info-box-icon {
    font-size: 1.25rem;
    flex-shrink: 0;
}
</style>