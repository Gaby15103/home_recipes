<script setup lang="ts">
import { ref, reactive } from 'vue';
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";

const props = defineProps<{
  images: string[];
  defaultLang: string;
}>();

const emit = defineEmits(['zones-completed', 'cancel']);

const canvasRefs = ref<HTMLCanvasElement[]>([]);
const imageRefs = ref<HTMLImageElement[]>([]);
const sourceLang = ref(props.defaultLang);
const currentType = ref('ingredients');

const history = ref<string[]>([]);
const redoStack = ref<string[]>([]);

const regions = reactive<any[]>([]);
const isDrawing = ref(false);
const isMoving = ref(false);
const selectedIdx = ref<number | null>(null);
const dragStart = reactive({ x: 0, y: 0 });

const colors: Record<string, string> = {
  title: '#ef4444',
  ingredients: '#22c55e',
  steps: '#3b82f6'
};

const saveHistory = () => {
  history.value.push(JSON.stringify(regions));
  if (history.value.length > 20) history.value.shift();
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

const getMousePos = (e: MouseEvent, idx: number) => {
  const canvas = canvasRefs.value[idx];
  const rect = canvas.getBoundingClientRect();
  const scaleX = canvas.width / rect.width;
  const scaleY = canvas.height / rect.height;
  return {
    x: (e.clientX - rect.left) * scaleX,
    y: (e.clientY - rect.top) * scaleY
  };
};

const initCanvas = (idx: number) => {
  const img = imageRefs.value[idx];
  const canvas = canvasRefs.value[idx];
  if (!img || !canvas) return;
  canvas.width = img.naturalWidth;
  canvas.height = img.naturalHeight;
  renderAll();
};

const handleMouseDown = (e: MouseEvent, imgIdx: number) => {
  const pos = getMousePos(e, imgIdx);

  // Find if we clicked an existing region on THIS page
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
};

const handleMouseMove = (e: MouseEvent, imgIdx: number) => {
  if (selectedIdx.value === null) return;
  const r = regions[selectedIdx.value];
  if (r.image_index !== imgIdx) return;

  const pos = getMousePos(e, imgIdx);

  if (isDrawing.value) {
    r.x = Math.min(dragStart.x, pos.x);
    r.y = Math.min(dragStart.y, pos.y);
    r.w = Math.abs(pos.x - dragStart.x);
    r.h = Math.abs(pos.y - dragStart.y);
  } else if (isMoving.value) {
    r.x = pos.x - dragStart.x;
    r.y = pos.y - dragStart.y;
  }
  renderAll();
};

const handleMouseUp = () => {
  isDrawing.value = false;
  isMoving.value = false;
  // Clean up tiny accidental clicks
  if (selectedIdx.value !== null) {
    const r = regions[selectedIdx.value];
    if (r && r.w < 5 && r.h < 5) regions.splice(selectedIdx.value, 1);
  }
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

      // Draw Box
      ctx.fillStyle = colors[r.type] + (isSelected ? '77' : '33');
      ctx.fillRect(r.x, r.y, r.w, r.h);

      ctx.strokeStyle = colors[r.type];
      ctx.lineWidth = isSelected ? 8 : 4;
      ctx.strokeRect(r.x, r.y, r.w, r.h);

      // Label
      ctx.fillStyle = "white";
      ctx.font = "bold 24px sans-serif";
      ctx.shadowBlur = 4;
      ctx.shadowColor = "black";
      ctx.fillText(r.type.toUpperCase(), r.x + 10, r.y + 30);
      ctx.shadowBlur = 0;
    });
  });
};

const finish = () => {

  const cleanRegions = regions.map(r => ({
    ...r,
    x: Math.round(r.x),
    y: Math.round(r.y),
    w: Math.round(r.w),
    h: Math.round(r.h),
    image_index: Math.round(r.image_index)
  }));

  emit('zones-completed', {
    regions: JSON.parse(JSON.stringify(cleanRegions)),
    sourceLang: sourceLang.value
  });
};
</script>

<template>
  <div class="flex flex-col h-full bg-zinc-950 select-none">
    <div class="flex items-center justify-between p-3 border-b border-zinc-800 bg-zinc-900 shrink-0 sticky top-0 z-50">
      <div class="flex items-center gap-2">
        <button v-for="t in ['title', 'ingredients', 'steps']" :key="t"
                @click="currentType = t"
                :class="['px-4 py-1.5 text-xs font-bold uppercase rounded-md transition-all border',
                currentType === t ? 'bg-white text-black border-white' : 'text-zinc-400 bg-zinc-800 border-zinc-700 hover:bg-zinc-700']">
          {{ t }}
        </button>
      </div>

      <div class="flex items-center gap-2">
        <Button variant="outline" size="sm" @click="undo" :disabled="history.length === 0" class="h-8 text-xs">Undo</Button>
        <Button variant="outline" size="sm" @click="redo" :disabled="redoStack.length === 0" class="h-8 text-xs">Redo</Button>
        <Separator orientation="vertical" class="h-6 mx-2 bg-zinc-700" />
        <Button @click="finish" class="bg-blue-600 hover:bg-blue-700 h-8 px-6 text-xs font-bold uppercase tracking-wider">
          Process Recipe
        </Button>
      </div>
    </div>

    <div class="flex-1 overflow-y-auto p-12 space-y-8 bg-zinc-950">
      <div v-for="(img, idx) in images" :key="idx" class="flex flex-col items-center">
        <div class="w-full max-w-4xl relative group">
          <div class="absolute -top-6 left-0 text-[10px] font-mono text-zinc-500 uppercase tracking-tighter">
            Source Image Part {{ idx + 1 }}
          </div>
          <div class="relative bg-white shadow-[0_20px_50px_rgba(0,0,0,0.5)] border border-zinc-800 overflow-hidden">
            <img
                :ref="el => imageRefs[idx] = (el as HTMLImageElement)"
                :src="img"
                @load="initCanvas(idx)"
                class="w-full h-auto block pointer-events-none"
            />
            <canvas
                :ref="el => canvasRefs[idx] = (el as HTMLCanvasElement)"
                @mousedown="handleMouseDown($event, idx)"
                @mousemove="handleMouseMove($event, idx)"
                @mouseup="handleMouseUp"
                @mouseleave="handleMouseUp"
                class="absolute top-0 left-0 w-full h-full cursor-crosshair touch-none"
            ></canvas>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
canvas {
  image-rendering: auto;
}
/* Visual indicator for multi-page flow */
.group:not(:last-child)::after {
  content: '';
  position: absolute;
  bottom: -40px;
  left: 50%;
  transform: translateX(-50%);
  width: 2px;
  height: 24px;
  background: repeating-linear-gradient(to bottom, #3f3f46, #3f3f46 4px, transparent 4px, transparent 8px);
}
</style>