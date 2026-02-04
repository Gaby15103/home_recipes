<script setup lang="ts">
import {Button} from "@/components/ui/button"
import {Input} from "@/components/ui/input"
import type {IngredientCreate, IngredientGroupCreate} from "@/models/RecipeCreate.ts";
import {IngredientUnit} from "@/models/Recipe.ts";
import IngredientUnitSelect from "@/components/Recipe/IngredientUnitSelect.vue";
import {Textarea} from "@/components/ui/textarea";
import {useI18n} from "vue-i18n";
const { t } = useI18n()
const props = defineProps<{
  modelValue: IngredientGroupCreate[]
}>()

const emit = defineEmits(["update:modelValue"])

function addGroup() {
  emit("update:modelValue", [
    ...props.modelValue,
    {
      title: "",
      position: props.modelValue.length,
      ingredients: [],
    },
  ])
}

function addIngredient(group: IngredientGroupCreate) {
  group.ingredients.push({
    name: "",
    quantity: 0,
    unit: IngredientUnit.Gram,
    note: null,
    position: group.ingredients.length,
  })
}

function removeIngredient(group: IngredientGroupCreate, ingredient: IngredientCreate) {
  const index = group.ingredients.indexOf(ingredient)
  if (index > -1) {
    group.ingredients.splice(index, 1)
  }
  group.ingredients.forEach((ing, i) => (ing.position = i))
}

function removeGroup(group: IngredientGroupCreate) {
  const index = props.modelValue.indexOf(group)
  if (index > -1) {
    const newGroups = [...props.modelValue]
    newGroups.splice(index, 1)
    emit("update:modelValue", newGroups)
  }
}

</script>

<template>
  <div class="space-y-6 mb-8">
    <div class="flex justify-between items-center">
      <h2 class="text-xl font-semibold">{{ t('Admin.ingredients.title') }}</h2>
      <Button size="sm" @click="addGroup">{{ t('Admin.ingredients.addGroup') }}</Button>
    </div>

    <div v-for="group in modelValue" class="border rounded p-4 space-y-3">
      <div class="grid grid-cols-2 gap-4">
        <Input :placeholder="t('Admin.ingredients.groupTitle')" class="w-[125%]" v-model="group.title"/>
        <Button size="sm" class="ml-[50%] w-[50%]" variant="outline" @click="removeGroup(group)">
          {{ t('Admin.ingredients.remove') }}
        </Button>
      </div>

      <div
          v-for="ingredient in group.ingredients"
          class="grid grid-cols-4 gap-2 mb-3 p-1 rounded-2xl border-1 border-solid border-gray-1"
      >
        <Input :placeholder="t('Admin.ingredients.name')" v-model="ingredient.name"/>
        <Input type="number" v-model.number="ingredient.quantity"/>
        <IngredientUnitSelect v-model="ingredient.unit"/>
        <Button size="sm" class="ml-[50%] w-[50%]" variant="outline" @click="removeIngredient(group, ingredient)">
          {{ t('Admin.ingredients.remove') }}
        </Button>
        <div class="col-span-4 mt-1">
          <Label class="text-sm text-gray-500">{{ t('Admin.ingredients.note') }}</Label>
          <Textarea
              :placeholder="t('Admin.ingredients.note_placeholder')"
              v-model="ingredient.note"
              class="w-full"
              rows="2"
          />
        </div>
      </div>

      <Button size="sm" variant="outline" @click="addIngredient(group)">
        {{ t('Admin.ingredients.addIngredient') }}
      </Button>
    </div>
  </div>
</template>
