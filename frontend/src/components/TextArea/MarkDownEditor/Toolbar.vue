<script setup lang="ts">
import ToolbarButton from "./ToolbarButton.vue"
import { defineEmits } from "vue"

const emit = defineEmits<{
  (e: "action", action: string): void
}>()
</script>

<div class="flex flex-wrap gap-1 mb-2">
<ToolbarButton label="B" @click="emit('action', '**')"/>
<ToolbarButton label="I" @click="emit('action', '*')"/>
<ToolbarButton label="H1" @click="emit('action', '# ')"/>
<ToolbarButton label="H2" @click="emit('action', '## ')"/>
<ToolbarButton label="Quote" @click="emit('action', '> ')"/>
<ToolbarButton label="Code" @click="emit('action', '```')"/>
<ToolbarButton label="Link" @click="emit('action', '[text](url)')"/>
<ToolbarButton label="Image" @click="emit('action', '![alt](url)')"/>
<ToolbarButton label="List" @click="emit('action', '- ')"/>
<ToolbarButton label="Check" @click="emit('action', '- [ ] ')"/>
<ToolbarButton label="Table" @click="emit('action', '| Header | Header |\n| ------ | ------ |\n| Cell   | Cell   |')"/>
</div>
