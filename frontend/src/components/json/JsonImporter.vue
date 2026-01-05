<script setup lang="ts" generic="T extends object">
import {ref} from "vue"
import {Button} from "@/components/ui/button"
import JsonTextarea from "@/components/json/JsonTextarea.vue"
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card"
import {Input} from "@/components/ui/input"
import {Accordion, AccordionItem, AccordionContent, AccordionTrigger} from "@/components/ui/accordion";

// Props
const props = defineProps<{
  modelValue: T
}>()

const emit = defineEmits<{
  (e: "update:modelValue", value: T): void
}>()

// Local JSON string for textarea
const importJson = ref(JSON.stringify(props.modelValue, null, 2))

/* -----------------------------
   Import logic
----------------------------- */

// Apply imported object (emit to parent)
function importObject(obj: T) {
  emit("update:modelValue", structuredClone(obj))
  importJson.value = JSON.stringify(obj, null, 2)
}

// Import from textarea
function importFromText() {
  try {
    const parsed = JSON.parse(importJson.value) as T
    importObject(parsed)
  } catch {
    alert("Invalid JSON")
  }
}

// Import from file
function onImportFile(e: Event) {
  const file = (e.target as HTMLInputElement).files?.[0]
  if (!file) return

  const reader = new FileReader()
  reader.onload = () => {
    try {
      const text = reader.result as string
      importJson.value = text
      const parsed = JSON.parse(text) as T
      importObject(parsed)
    } catch {
      alert("Invalid JSON file")
    }
  }
  reader.readAsText(file)
}
</script>

<template>
  <Accordion type="single" collapsible>
    <AccordionItem value="item-1">
      <Card>
        <AccordionTrigger class="px-3 py-1 text-sm h-auto">
          <CardHeader>
            <CardTitle><span class="whitespace-nowrap">Import JSON</span></CardTitle>
          </CardHeader>
        </AccordionTrigger>
        <AccordionContent>


          <CardContent class="space-y-3">
            <!-- File import -->
            <Input
                type="file"
                accept="application/json"
                @change="onImportFile"
            />

            <!-- Paste JSON -->
            <JsonTextarea
                v-model="importJson"
                placeholder="Paste JSON here…"
            />

            <Button size="sm" variant="outline" @click="importFromText">
              Import JSON
            </Button>
          </CardContent>

        </AccordionContent>
      </Card>
    </AccordionItem>
  </Accordion>
</template>
