<script setup lang="ts">
import {onMounted, onUnmounted, reactive, ref} from 'vue';
import {Button} from "@/components/ui/button";
import {Redo2, RotateCcw, Undo2, X, ZoomIn, ZoomOut} from "lucide-vue-next";

const props = defineProps<{
  images: string[];
  defaultLang?: string;
}>();

const emit = defineEmits(['zones-completed', 'cancel']);

// --- Refs & State ---
const canvasRefs = ref<HTMLCanvasElement[]>([]);
const imageRefs = ref<HTMLImageElement[]>([]);
const scrollContainer = ref<HTMLElement | null>(null);

const sourceLang = ref(props.defaultLang || 'fr');
const currentType = ref('title');
const zoomLevel = ref(1);

const history = ref<string[]>([]);
const redoStack = ref<string[]>([]);
const regions = reactive<any[]>([]);

// Tracking state
const activeImgIdx = ref<number | null>(null);
const isDrawing = ref(false);
const isMoving = ref(false);
const selectedIdx = ref<number | null>(null);
const dragStart = reactive({ x: 0, y: 0 });

const colors: Record<string, string> = {
  title: '#ef4444',
  ingredients: '#22c55e',
  steps: '#3b82f6'
};

// --- Logic ---

const saveHistory = () => {
  history.value.push(JSON.stringify(regions));
  if (history.value.length > 30) history.value.shift();
  redoStack.value = [];
};

const undo = () => {
  if (history.value.length === 0) return;
  redoStack.value.push(JSON.stringify(regions));
  const prev = JSON.parse(history.value.pop()!);
  regions.splice(0, regions.length, ...prev);
  renderAll();
};

const redo = () => {
  if (redoStack.value.length === 0) return;
  history.value.push(JSON.stringify(regions));
  const next = JSON.parse(redoStack.value.pop()!);
  regions.splice(0, regions.length, ...next);
  renderAll();
};

const getPointerPos = (e: PointerEvent, idx: number) => {
  const canvas = canvasRefs.value[idx];
  if (!canvas) return { x: 0, y: 0 };
  const rect = canvas.getBoundingClientRect();

  // Account for CSS scaling/zoom
  const scaleX = canvas.width / rect.width;
  const scaleY = canvas.height / rect.height;

  return {
    x: (e.clientX - rect.left) * scaleX,
    y: (e.clientY - rect.top) * scaleY
  };
};

const handlePointerDown = (e: PointerEvent, imgIdx: number) => {
  // Only handle primary button or touch
  if (e.button !== 0 && e.pointerType === 'mouse') return;

  const pos = getPointerPos(e, imgIdx);
  activeImgIdx.value = imgIdx;

  const hitIdx = regions.findIndex(r =>
      r.image_index === imgIdx &&
      pos.x >= r.x && pos.x <= r.x + r.w &&
      pos.y >= r.y && pos.y <= r.y + r.h
  );

  saveHistory();

  if (hitIdx !== -1) {
    selectedIdx.value = hitIdx;
    isMoving.value = true;
    dragStart.x = pos.x - regions[hitIdx].x;
    dragStart.y = pos.y - regions[hitIdx].y;
  } else {
    selectedIdx.value = regions.length;
    isDrawing.value = true;
    dragStart.x = pos.x;
    dragStart.y = pos.y;
    regions.push({
      x: pos.x, y: pos.y, w: 0, h: 0,
      type: currentType.value,
      image_index: imgIdx
    });
  }

  // Lock pointer to capture movement outside canvas
  (e.target as HTMLElement).setPointerCapture(e.pointerId);
};

const handleGlobalPointerMove = (e: PointerEvent) => {
  if (selectedIdx.value === null || activeImgIdx.value === null) return;

  const imgIdx = activeImgIdx.value;
  const r = regions[selectedIdx.value];
  const pos = getPointerPos(e, imgIdx);

  if (isDrawing.value) {
    r.x = Math.min(dragStart.x, pos.x);
    r.y = Math.min(dragStart.y, pos.y);
    r.w = Math.abs(pos.x - dragStart.x);
    r.h = Math.abs(pos.y - dragStart.y);
  } else if (isMoving.value) {
    r.x = pos.x - dragStart.x;
    r.y = pos.y - dragStart.y;
  }

  // Auto-scroll logic
  if (scrollContainer.value) {
    const threshold = 50;
    const scrollSpeed = 10;
    if (e.clientY < threshold) scrollContainer.value.scrollTop -= scrollSpeed;
    if (e.clientY > window.innerHeight - threshold) scrollContainer.value.scrollTop += scrollSpeed;
  }

  renderAll();
};

const handleGlobalPointerUp = () => {
  if (selectedIdx.value !== null) {
    const r = regions[selectedIdx.value];
    // Remove if tiny (accidental click)
    if (r && r.w < 10 && r.h < 10) regions.splice(selectedIdx.value, 1);
  }
  isDrawing.value = false;
  isMoving.value = false;
  selectedIdx.value = null;
  activeImgIdx.value = null;
  renderAll();
};

const renderAll = () => {
  canvasRefs.value.forEach((canvas, idx) => {
    if (!canvas) return;
    const ctx = canvas.getContext('2d')!;
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    regions.forEach((r, globalIdx) => {
      if (r.image_index !== idx) return;
      const isSelected = globalIdx === selectedIdx.value;

      ctx.fillStyle = colors[r.type] + (isSelected ? '66' : '33');
      ctx.fillRect(r.x, r.y, r.w, r.h);

      ctx.strokeStyle = colors[r.type];
      ctx.lineWidth = Math.max(4, 4 / zoomLevel.value);
      ctx.strokeRect(r.x, r.y, r.w, r.h);
    });
  });
};

const initCanvas = (idx: number) => {
  const img = imageRefs.value[idx];
  const canvas = canvasRefs.value[idx];
  if (!img || !canvas) return;
  canvas.width = img.naturalWidth;
  canvas.height = img.naturalHeight;
  renderAll();
};

onMounted(() => {
  window.addEventListener('pointermove', handleGlobalPointerMove);
  window.addEventListener('pointerup', handleGlobalPointerUp);
});

onUnmounted(() => {
  window.removeEventListener('pointermove', handleGlobalPointerMove);
  window.removeEventListener('pointerup', handleGlobalPointerUp);
});

const finish = () => {
  const cleanRegions = regions.map(r => ({
    x: Math.round(r.x),
    y: Math.round(r.y),
    w: Math.round(r.w),
    h: Math.round(r.h),
    label: r.type,
    image_index: r.image_index
  }));

  emit('zones-completed', {
    regions: cleanRegions,
    sourceLang: sourceLang.value
  });
};
</script>

<template>
  <div class="flex flex-col h-full bg-zinc-950 select-none overflow-hidden touch-none">
    <div class="z-50 flex flex-col md:flex-row items-center justify-between p-3 gap-4 border-b border-zinc-800 bg-zinc-900 shrink-0">

      <div class="flex items-center gap-2 overflow-x-auto no-scrollbar w-full md:w-auto">
        <button v-for="t in ['title', 'ingredients', 'steps']" :key="t"
                @click="currentType = t"
                :class="['px-4 py-2 text-[10px] font-black uppercase rounded-xl transition-all border flex items-center gap-2 shrink-0',
                currentType === t ? 'bg-white text-black border-white' : 'text-zinc-400 bg-zinc-800 border-zinc-700']">
          <div class="w-2 h-2 rounded-full" :style="{ backgroundColor: colors[t] }"></div>
          {{ t }}
        </button>
      </div>

      <div class="flex items-center gap-3 w-full md:w-auto justify-between md:justify-end">
        <div class="flex items-center gap-1 bg-zinc-800 p-1 rounded-xl">
          <Button variant="ghost" size="icon" class="h-8 w-8" @click="zoomLevel = Math.max(0.5, zoomLevel - 0.25)"><ZoomOut class="w-4 h-4" /></Button>
          <span class="text-[10px] font-black w-10 text-center">{{ Math.round(zoomLevel * 100) }}%</span>
          <Button variant="ghost" size="icon" class="h-8 w-8" @click="zoomLevel = Math.min(3, zoomLevel + 0.25)"><ZoomIn class="w-4 h-4" /></Button>
        </div>

        <div class="flex items-center gap-2">
          <Button variant="outline" size="icon" @click="regions.length = 0" class="h-8 w-8 border-zinc-700 text-zinc-400"><RotateCcw class="w-4 h-4" /></Button>
          <Button @click="finish" class="bg-blue-600 hover:bg-blue-700 h-9 px-4 text-[10px] font-black uppercase tracking-widest rounded-xl">
            Confirm
          </Button>
          <Button variant="ghost" size="icon" @click="emit('cancel')" class="h-9 w-9 text-zinc-500"><X /></Button>
        </div>
      </div>
    </div>

    <div ref="scrollContainer" class="flex-1 overflow-auto bg-zinc-950 p-4 md:p-20 flex flex-col items-center gap-10">
      <div v-for="(img, idx) in images" :key="idx"
           class="relative transition-transform duration-200 origin-top"
           :style="{ transform: `scale(${zoomLevel})` }">

        <div class="relative bg-white shadow-2xl border border-zinc-800 overflow-hidden rounded-sm">
          <img
              :ref="el => imageRefs[idx] = (el as HTMLImageElement)"
              :src="img"
              @load="initCanvas(idx)"
              class="max-w-none block pointer-events-none"
              style="width: 800px; height: auto;"
          />
          <canvas
              :ref="el => canvasRefs[idx] = (el as HTMLCanvasElement)"
              @pointerdown="handlePointerDown($event, idx)"
              class="absolute top-0 left-0 w-full h-full cursor-crosshair touch-none"
          ></canvas>
        </div>

        <div class="absolute -top-6 left-0 text-[9px] font-black uppercase tracking-widest text-zinc-500">
          Page {{ idx + 1 }}
        </div>
      </div>
    </div>

    <div class="p-4 border-t border-zinc-800 bg-zinc-900/50 backdrop-blur-md flex justify-center gap-10">
      <button @click="undo" :disabled="history.length === 0" class="text-zinc-400 disabled:opacity-20"><Undo2 /></button>
      <button @click="redo" :disabled="redoStack.length === 0" class="text-zinc-400 disabled:opacity-20"><Redo2 /></button>
    </div>
  </div>
</template>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
</style>