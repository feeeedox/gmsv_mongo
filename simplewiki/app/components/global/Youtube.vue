<template>
  <div class="nuxt-content-youtube-wrapper">
    <ScriptYouTubePlayer :video-id="id" :thumbnail-size="thumbSize" trigger="mousedown" class="custom-youtube-player">
      <div class="play-button-overlay">
        <div class="play-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
            <path d="M8 5v14l11-7z" />
          </svg>
        </div>
      </div>
    </ScriptYouTubePlayer>
  </div>
</template>

<script setup>
defineProps({
  id: {
    type: String,
    required: true
  },
  thumbSize: {
    type: String,
    default: 'maxresdefault'
  }
})
</script>

<style scoped>
.nuxt-content-youtube-wrapper {
  position: relative;
  width: 100%;
  border-radius: 8px;
  overflow: hidden;
  margin: 1.5rem 0;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  aspect-ratio: 16 / 9;
  background-color: #000;
}

.custom-youtube-player {
  width: 100%;
  height: 100%;
  position: relative;
  display: block;
}

.play-button-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.25);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: background 0.3s ease;
  z-index: 10;
  pointer-events: none;
}

.custom-youtube-player:hover .play-button-overlay {
  background: rgba(0, 0, 0, 0.4);
}

.play-icon {
  width: 68px;
  height: 48px;
  background-color: rgba(33, 33, 33, 0.9);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  transition: background-color 0.2s cubic-bezier(0, 0, 0.2, 1), transform 0.2s ease;
}

.custom-youtube-player:hover .play-icon {
  background-color: #ff0000;
  transform: scale(1.05);
}

.play-icon svg {
  width: 30px;
  height: 30px;
}

:deep(iframe)~.play-button-overlay,
.custom-youtube-player:has(iframe) .play-button-overlay {
  display: none !important;
}
</style>