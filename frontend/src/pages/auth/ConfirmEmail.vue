<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { confirmEmail } from "@/api";
import { useI18n } from "vue-i18n"
const { t } = useI18n()
const route = useRoute();
const token = route.query.token as string;

const loading = ref(true);
const success = ref(false);
const message = ref("");

onMounted(async () => {
  if (!token) {
    loading.value = false;
    success.value = false;
    message.value = t('auth.confirmEmail.invalidLink');
    return;
  }

  try {
    const res = await confirmEmail(token);
    loading.value = false;
    success.value = res.success;
    message.value = res.message;
  } catch (err: any) {
    loading.value = false;
    success.value = false;
    message.value = err?.response?.data?.message || t('auth.confirmEmail.error');
  }
});
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full text-center">
      <h2 class="text-3xl font-extrabold text-gray-900 mb-6">{{ t('auth.confirmEmail.title') }}</h2>

      <div v-if="loading" class="text-gray-500">
        {{ t('auth.confirmEmail.loading') }}
      </div>

      <div v-else>
        <div v-if="success" class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4">
          {{ message }}
        </div>
        <div v-else class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
          {{ message }}
        </div>

        <router-link v-if="success" to="/login" class="inline-block mt-4 px-4 py-2 bg-indigo-600 text-white rounded hover:bg-indigo-500">
          {{ t('auth.confirmEmail.goLogin') }}
        </router-link>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Optional: add some subtle animation */
</style>
