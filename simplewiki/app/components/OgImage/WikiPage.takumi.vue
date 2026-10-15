<script setup lang="ts">
withDefaults(defineProps<{
  title?: string
  description?: string
}>(), {
  title: 'My Page Title',
  description: 'My page description',
})
</script>

<template>
  <div class="og-container">
    <div class="og-content">
      <h1 class="og-title">
        {{ title }}
      </h1>
      
      <p class="og-description">
        {{ description }}
      </p>
    </div>

    <div class="og-divider" />
  </div>
</template>

<style scoped>
.og-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  background-color: #f8f9fa;
  padding: 64px; 
  border-left: 12px solid #a2a9b1;
  border-right: 12px solid #a2a9b1;
  box-sizing: border-box; 
}

.og-content {
  display: flex;
  flex-direction: column;
  gap: 24px; 
}

.og-title {
  font-family: Georgia, Cambria, "Times New Roman", Times, serif; 
  font-size: 60px;
  font-weight: normal;
  color: #000000;
  padding-bottom: 8px; 
  border-bottom: 1px solid #a2a9b1;
  line-height: 1.25; 
  margin: 0;
}

.og-description {
  font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif; /* font-sans */
  font-size: 26px;
  color: #202122;
  line-height: 1.625;
  max-width: 90%;
  margin: 0;
}

.og-divider {
  width: 100%;
  border-top: 1px solid #eaecf0;
}
</style>