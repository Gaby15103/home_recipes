<template>
  <Dialog v-model:open="open">
    <DialogContent class="max-w-md space-y-6">
      <DialogHeader>
        <DialogTitle>Print Options</DialogTitle>
      </DialogHeader>

      <div class="space-y-4">
        <div class="flex items-center gap-2">
          <Checkbox id="main-image" v-model="includeMainImage" />
          <Label for="main-image" class="cursor-pointer">Include main image</Label>
        </div>

        <div class="flex items-center gap-2">
          <Checkbox id="step-images" v-model="includeStepImages" />
          <Label for="step-images" class="cursor-pointer">Include step images</Label>
        </div>

        <div class="flex items-center gap-2">
          <Checkbox id="tags" v-model="includeTags" />
          <Label for="tags" class="cursor-pointer">Include tags</Label>
        </div>
      </div>

      <DialogFooter class="flex justify-end gap-2">
        <Button variant="outline" @click="closeModal">Cancel</Button>
        <Button variant="default" @click="confirmPrint">Print</Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, defineExpose, defineProps } from "vue";
import { printRecipe } from "@/utils/RecipePrinter";
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Label } from "@/components/ui/label";
import type { Recipe } from "@/models/Recipe";

// Props
const props = defineProps<{ recipe: Recipe }>();

// Modal state
const open = ref(false);

// Options
const includeMainImage = ref(true);
const includeStepImages = ref(true);
const includeTags = ref(true);

// Print function
function confirmPrint() {
  if (props.recipe) {
    printRecipe(props.recipe, {
      includeMainImage: includeMainImage.value,
      includeStepImages: includeStepImages.value,
      includeTags: includeTags.value,
    });
    closeModal();
  }
}

// Open/close modal
function showModal() {
  open.value = true;
}
function closeModal() {
  open.value = false;
}

defineExpose({ showModal, closeModal });
</script>
