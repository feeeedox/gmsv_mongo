<script lang="ts" setup>
import {computed, nextTick, onMounted, ref} from 'vue'
import {renderMermaid} from '~/util/mermaidRenderer.ts'

const slotContainerRef = ref<HTMLDivElement | null>(null)

function extractSource(): string {
  const raw = slotContainerRef.value?.textContent ?? ''
  return raw.replace(/^\s*mermaid\s*\n/, '').trim()
}

const svgContent  = ref('')
const renderError = ref('')
const isLoading   = ref(true)

const containerRef = ref<HTMLDivElement | null>(null)
const canvasRef    = ref<HTMLDivElement | null>(null)
const transform    = ref({ x: 0, y: 0, scale: 1 })

const MIN_SCALE   = 0.15
const MAX_SCALE   = 8
const ZOOM_FACTOR = 0.12

let isPanning     = false
let panStart      = { x: 0, y: 0 }
let originAtStart = { x: 0, y: 0 }

function clampScale(s: number) {
  return Math.min(MAX_SCALE, Math.max(MIN_SCALE, s))
}

function applyZoom(delta: number, cx: number, cy: number) {
  const t     = transform.value
  const next  = clampScale(t.scale * (1 + delta))
  const ratio = next / t.scale
  transform.value = {
    scale: next,
    x: cx - ratio * (cx - t.x),
    y: cy - ratio * (cy - t.y),
  }
}

function onWheel(e: WheelEvent) {
  e.preventDefault()
  const rect  = containerRef.value!.getBoundingClientRect()
  const cx    = e.clientX - rect.left
  const cy    = e.clientY - rect.top
  applyZoom(e.deltaY < 0 ? ZOOM_FACTOR : -ZOOM_FACTOR, cx, cy)
}

function onPointerDown(e: PointerEvent) {
  if (e.button !== 0 && e.pointerType === 'mouse') return
  isPanning     = true
  panStart      = { x: e.clientX, y: e.clientY }
  originAtStart = { ...transform.value }
  ;(e.currentTarget as HTMLElement).setPointerCapture(e.pointerId)
}

function onPointerMove(e: PointerEvent) {
  if (!isPanning) return
  transform.value = {
    ...transform.value,
    x: originAtStart.x + (e.clientX - panStart.x),
    y: originAtStart.y + (e.clientY - panStart.y),
  }
}

function onPointerUp() { isPanning = false }

let lastPinchDist = 0

function getTouchDist(e: TouchEvent) {
  const [a, b] = [e.touches[0], e.touches[1]]
  // @ts-ignore
  return Math.hypot(b.clientX - a.clientX, b.clientY - a.clientY)
}

function onTouchStart(e: TouchEvent) {
  if (e.touches.length === 2) { lastPinchDist = getTouchDist(e); e.preventDefault() }
}

function onTouchMove(e: TouchEvent) {
  if (e.touches.length !== 2) return
  e.preventDefault()
  const dist  = getTouchDist(e)
  const rect  = containerRef.value!.getBoundingClientRect()
  // @ts-ignore
  const cx    = ((e.touches[0].clientX + e.touches[1].clientX) / 2) - rect.left
  // @ts-ignore
  const cy    = ((e.touches[0].clientY + e.touches[1].clientY) / 2) - rect.top
  applyZoom((dist - lastPinchDist) / lastPinchDist * 0.6, cx, cy)
  lastPinchDist = dist
}

function zoomIn()  { applyZoom( ZOOM_FACTOR * 3, containerRef.value!.clientWidth / 2, containerRef.value!.clientHeight / 2) }
function zoomOut() { applyZoom(-ZOOM_FACTOR * 3, containerRef.value!.clientWidth / 2, containerRef.value!.clientHeight / 2) }

function resetView() {
  if (!containerRef.value || !canvasRef.value) return
  const svg = canvasRef.value.querySelector('svg')
  if (!svg) { transform.value = { x: 0, y: 0, scale: 1 }; return }

  const cw = containerRef.value.clientWidth
  const ch = containerRef.value.clientHeight
  const vb = svg.viewBox.baseVal
  const sw = vb.width  || svg.getBoundingClientRect().width  || 800
  const sh = vb.height || svg.getBoundingClientRect().height || 600

  const scale = clampScale(Math.min((cw - 48) / sw, (ch - 48) / sh))
  transform.value = { scale, x: (cw - sw * scale) / 2, y: (ch - sh * scale) / 2 }
}

onMounted(async () => {
  try {
    svgContent.value  = await renderMermaid(extractSource())
    renderError.value = ''
  } catch (err: any) {
    renderError.value = err?.message ?? 'Render failed'
  } finally {
    isLoading.value = false
    await nextTick()
    resetView()
  }
})

const zoomPercent = computed(() => Math.round(transform.value.scale * 100))

const canvasStyle = computed(() => ({
  transform: `translate(${transform.value.x}px, ${transform.value.y}px) scale(${transform.value.scale})`,
  transformOrigin: '0 0',
}))
</script>

<template>
  <div class="mermaid-wrapper select-none!">
    <div ref="slotContainerRef" aria-hidden="true" style="display:none">
      <slot />
    </div>

    <div class="mermaid-header">
      <span class="mermaid-label">
        <svg fill="none" height="13" viewBox="0 0 24 24" width="13">
          <rect height="5" rx="1" stroke="currentColor" stroke-width="2" width="7" x="2"  y="2"/>
          <rect height="5" rx="1" stroke="currentColor" stroke-width="2" width="7" x="15" y="9"/>
          <rect height="5" rx="1" stroke="currentColor" stroke-width="2" width="7" x="2"  y="17"/>
          <path d="M9 4.5h3.5a1 1 0 0 1 1 1v5"  stroke="currentColor" stroke-linecap="round" stroke-width="2"/>
          <path d="M9 19.5h3.5a1 1 0 0 0 1-1v-5" stroke="currentColor" stroke-linecap="round" stroke-width="2"/>
        </svg>
        Diagram
      </span>
      <div class="mermaid-controls">
        <span class="zoom-level">{{ zoomPercent }}%</span>
        <button class="ctrl-btn" title="Zoom in" @click="zoomIn">
          <svg fill="none" height="13" viewBox="0 0 24 24" width="13">
            <circle cx="11" cy="11" r="8" stroke="currentColor" stroke-width="2"/>
            <path d="M21 21l-4.35-4.35M11 8v6M8 11h6" stroke="currentColor" stroke-linecap="round" stroke-width="2"/>
          </svg>
        </button>
        <button class="ctrl-btn" title="Zoom out" @click="zoomOut">
          <svg fill="none" height="13" viewBox="0 0 24 24" width="13">
            <circle cx="11" cy="11" r="8" stroke="currentColor" stroke-width="2"/>
            <path d="M21 21l-4.35-4.35M8 11h6" stroke="currentColor" stroke-linecap="round" stroke-width="2"/>
          </svg>
        </button>
        <button class="ctrl-btn" title="Reset view" @click="resetView">
          <svg fill="none" height="13" viewBox="0 0 24 24" width="13">
            <path d="M3.5 12a8.5 8.5 0 1 1 1.8 5.3" stroke="currentColor" stroke-linecap="round" stroke-width="2"/>
            <path d="M3.5 17V12h5" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"/>
          </svg>
        </button>
      </div>
    </div>

    <div
        ref="containerRef"
        class="mermaid-viewport"
        @pointercancel="onPointerUp"
        @pointerdown="onPointerDown"
        @pointermove="onPointerMove"
        @pointerup="onPointerUp"
        @touchmove.prevent="onTouchMove"
        @touchstart.passive="onTouchStart"
        @wheel.prevent="onWheel"
    >
      <div v-if="isLoading" class="mermaid-status">
        <span class="spinner" />
        Rendering…
      </div>

      <div v-else-if="renderError" class="mermaid-status mermaid-error">
        <svg fill="none" height="16" viewBox="0 0 24 24" width="16">
          <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/>
          <path d="M12 8v4M12 16h.01" stroke="currentColor" stroke-linecap="round" stroke-width="2"/>
        </svg>
        <span>{{ renderError }}</span>
      </div>

      <div
          v-else
          ref="canvasRef"
          :style="canvasStyle"
          class="mermaid-canvas"
          v-html="svgContent"
      />

      <div v-if="!isLoading && !renderError" class="scroll-hint">
        scroll to zoom · drag to pan
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.mermaid-wrapper {
  margin: 16px 0;
  border: 1px solid var(--color-border-light);
  border-radius: 6px;
  overflow: hidden;
  background: var(--color-content-bg);
}

.mermaid-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 32px;
  padding: 0 10px 0 12px;
  background: var(--color-header-band);
  border-bottom: 1px solid var(--color-border-light);
  color: var(--color-text-muted);
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  font-size: 12px;
  font-weight: 500;
  user-select: none;
}

.mermaid-label {
  display: flex;
  align-items: center;
  gap: 6px;
}

.mermaid-controls {
  display: flex;
  align-items: center;
  gap: 2px;
}

.zoom-level {
  font-size: 11px;
  min-width: 34px;
  text-align: right;
  margin-right: 4px;
  color: var(--color-text-muted);
  opacity: 0.7;
}

.ctrl-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  padding: 0;
  background: transparent;
  border: 1px solid transparent;
  border-radius: 4px;
  color: var(--color-text-muted);
  cursor: pointer;
  transition: background 0.15s, border-color 0.15s, color 0.15s;

  &:hover {
    background: var(--color-content-bg);
    border-color: var(--color-border);
    color: var(--color-text);
  }

  &:focus-visible {
    outline: 2px solid var(--color-focus);
    outline-offset: 1px;
  }
}

.mermaid-viewport {
  position: relative;
  height: 380px;
  overflow: hidden;
  cursor: grab;
  touch-action: none;
  background:
      radial-gradient(circle, var(--color-border-light) 1px, transparent 1px) 0 0 / 20px 20px;

  &:active { cursor: grabbing; }
}

.mermaid-canvas {
  position: absolute;
  top: 0;
  left: 0;

  :deep(svg) {
    display: block;
    width: 100%;
    height: auto;
    max-width: none;

    text, .label {
      fill: var(--color-text) !important;
    }
  }
}

.mermaid-status {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  color: var(--color-text-muted);
  font-family: var(--font-body);
  font-size: 13px;
  pointer-events: none;
}

.mermaid-error {
  color: #c0392b;
  .dark & { color: #ff7675; }
}

.spinner {
  display: inline-block;
  width: 14px;
  height: 14px;
  border: 2px solid var(--color-border);
  border-top-color: var(--color-text-muted);
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.scroll-hint {
  position: absolute;
  bottom: 8px;
  left: 50%;
  transform: translateX(-50%);
  padding: 3px 8px;
  border-radius: 999px;
  background: var(--color-header-band);
  border: 1px solid var(--color-border-light);
  color: var(--color-text-muted);
  font-family: var(--font-body);
  font-size: 11px;
  opacity: 0.6;
  pointer-events: none;
  white-space: nowrap;
  user-select: none;
}
</style>