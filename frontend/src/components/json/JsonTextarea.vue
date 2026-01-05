<script setup lang="ts">
import {ref, watch} from "vue"
import {Textarea} from "@/components/ui/textarea"
import {Button} from "@/components/ui/button"

const props = defineProps<{
  modelValue: string
  placeholder?: string
  rows?: number
}>()

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void
}>()

const localValue = ref(props.modelValue)
const isInvalid = ref(false)

const errorDetails = ref<{
  message: string
  line: number
  column: number
  lineText: string
} | null>(null)


const INDENT = "  "

watch(
    () => props.modelValue,
    v => (localValue.value = v)
)

function updateLocal(v: string) {
  localValue.value = v
}

function commit(v: string) {
  localValue.value = v
  emit("update:modelValue", v)
}

/* -----------------------------
   Helpers
----------------------------- */

function getLineIndent(value: string, pos: number): string {
  const lineStart = value.lastIndexOf("\n", pos - 1) + 1
  return value.slice(lineStart, pos).match(/^\s*/)?.[0] ?? ""
}

/* -----------------------------
   JSON formatting
----------------------------- */

function formatSilent() {
  try {
    commit(JSON.stringify(JSON.parse(localValue.value), null, 2))
    isInvalid.value = false
    errorDetails.value = null
  } catch (err: any) {
    isInvalid.value = true
    errorDetails.value = getJsonErrorDetails(localValue.value, err)
  }
}

function formatJson() {
  try {
    commit(JSON.stringify(JSON.parse(localValue.value), null, 2))
    isInvalid.value = false
    errorDetails.value = null
  } catch (err: any) {
    isInvalid.value = true
    errorDetails.value = getJsonErrorDetails(localValue.value, err)
  }
}

function getJsonErrorDetails(json: string, error: any) {
  const match = error.message.match(/position (\d+)/)
  if (!match) return null

  const pos = Number(match[1])
  const before = json.slice(0, pos)

  const line = before.split("\n").length
  const col = before.length - before.lastIndexOf("\n")

  const lineText = json.split("\n")[line - 1]

  return {
    message: error.message,
    position: pos,
    line,
    column: col,
    lineText,
  }
}


function onPaste() {
  requestAnimationFrame(formatSilent)
}

/* -----------------------------
   Auto-pairing & indentation
----------------------------- */

const PAIRS: Record<string, string> = {
  "{": "}",
  "[": "]",
  "(": ")",
  '"': '"',
  "'": "'",
}

function onKeydown(e: KeyboardEvent) {
  const el = e.target as HTMLTextAreaElement
  const start = el.selectionStart
  const end = el.selectionEnd
  const value = localValue.value

  /* -------- ENTER handling -------- */
  if (e.key === "Enter") {
    e.preventDefault()

    const indent = getLineIndent(value, start)
    const prev = value[start - 1]
    const next = value[start]

    // Between {} or []
    if (
        (prev === "{" && next === "}") ||
        (prev === "[" && next === "]")
    ) {
      const innerIndent = indent + INDENT

      updateLocal(
          value.slice(0, start) +
          "\n" +
          innerIndent +
          "\n" +
          indent +
          value.slice(start)
      )

      requestAnimationFrame(() => {
        el.selectionStart = el.selectionEnd =
            start + 1 + innerIndent.length
      })
      return
    }

    // Normal enter → keep indent
    updateLocal(
        value.slice(0, start) +
        "\n" +
        indent +
        value.slice(start)
    )

    requestAnimationFrame(() => {
      el.selectionStart = el.selectionEnd =
          start + 1 + indent.length
    })
    return
  }

  /* -------- TAB handling -------- */
  if (e.key === "Tab") {
    e.preventDefault()

    // Shift+Tab → outdent
    if (e.shiftKey) {
      if (value.slice(start - INDENT.length, start) === INDENT) {
        updateLocal(
            value.slice(0, start - INDENT.length) +
            value.slice(start)
        )
        requestAnimationFrame(() => {
          el.selectionStart = el.selectionEnd =
              start - INDENT.length
        })
      }
      return
    }

    updateLocal(
        value.slice(0, start) +
        INDENT +
        value.slice(end)
    )

    requestAnimationFrame(() => {
      el.selectionStart = el.selectionEnd =
          start + INDENT.length
    })
    return
  }

  /* -------- Auto-pairs -------- */
  const close = PAIRS[e.key]
  if (!close) return

  if (value[start] === close) {
    e.preventDefault()
    el.selectionStart = el.selectionEnd = start + 1
    return
  }

  e.preventDefault()

  if (start !== end) {
    updateLocal(
        value.slice(0, start) +
        e.key +
        value.slice(start, end) +
        close +
        value.slice(end)
    )

    requestAnimationFrame(() => {
      el.selectionStart = start + 1
      el.selectionEnd = end + 1
    })
    return
  }

  updateLocal(
      value.slice(0, start) +
      e.key +
      close +
      value.slice(end)
  )

  requestAnimationFrame(() => {
    el.selectionStart = el.selectionEnd = start + 1
  })
}
</script>

<template>
  <div class="space-y-2">
    <Textarea
        v-model="localValue"
        :rows="rows ?? 8"
        :placeholder="placeholder ?? 'Paste JSON here…'"
        class="font-mono text-sm leading-relaxed"
        @input="updateLocal(($event.target as HTMLTextAreaElement).value)"
        @keydown="onKeydown"
        @blur="formatSilent"
        @paste="onPaste"
    />
    <div v-if="errorDetails" class="rounded border border-red-500 bg-red-50 p-2 text-xs font-mono">
      <p class="text-red-600 font-semibold">
        JSON Error
      </p>
      <p>{{ errorDetails.message }}</p>
      <p>
        Line {{ errorDetails.line }}, Column {{ errorDetails.column }}
      </p>

      <pre class="mt-1 text-red-700">
          {{ errorDetails.lineText }}
          {{ " ".repeat(errorDetails.column - 1) }}^
      </pre>
    </div>


    <div class="flex justify-between items-center">
      <p v-if="isInvalid" class="text-xs text-red-500">
        Invalid JSON
      </p>

      <Button size="sm" variant="outline" @click="formatJson">
        Format JSON
      </Button>
    </div>
  </div>
</template>
