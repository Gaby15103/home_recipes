<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { confirmEmail } from "@/api";
import { useI18n } from "vue-i18n";
import { CheckCircle2, XCircle, Loader2, ArrowRight, MailCheck } from "lucide-vue-next";
import { Button } from "@/components/ui/button";

const { t } = useI18n();
const route = useRoute();
const token = route.query.token as string;

const loading = ref(true);
const success = ref(false);
const message = ref("");

onMounted(async () => {
  if (!token) {
    setTimeout(() => {
      loading.value = false;
      success.value = false;
      message.value = t('auth.confirmEmail.invalidLink');
    }, 800);
    return;
  }

  try {
    const res = await confirmEmail(token);
    success.value = res.success;
    message.value = res.message;
  } catch (err: any) {
    success.value = false;
    message.value = err?.response?.data?.message || t('auth.confirmEmail.error');
  } finally {
    loading.value = false;
  }
});
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-zinc-950 px-4 select-none">
    <div class="max-w-md w-full relative">
      <div class="absolute -top-20 -left-20 w-40 h-40 bg-blue-600/10 blur-[100px] rounded-full"></div>
      <div class="absolute -bottom-20 -right-20 w-40 h-40 bg-emerald-600/10 blur-[100px] rounded-full"></div>

      <div class="bg-zinc-900 border border-zinc-800 rounded-3xl p-8 md:p-12 shadow-2xl relative overflow-hidden flex flex-col items-center text-center">

        <div class="mb-8">
          <div class="w-20 h-20 rounded-2xl bg-zinc-800 border border-zinc-700 flex items-center justify-center shadow-inner">
            <MailCheck v-if="loading" class="w-10 h-10 text-zinc-500 animate-pulse" />
            <CheckCircle2 v-else-if="success" class="w-10 h-10 text-emerald-500" />
            <XCircle v-else class="w-10 h-10 text-red-500" />
          </div>
        </div>

        <h2 class="text-2xl font-black text-white tracking-tight mb-4">
          {{ t('auth.confirmEmail.title') }}
        </h2>

        <div class="min-h-[120px] w-full flex flex-col items-center justify-center">
          <div v-if="loading" class="flex flex-col items-center gap-4">
            <Loader2 class="w-6 h-6 text-blue-500 animate-spin" />
            <p class="text-zinc-500 text-xs font-black uppercase tracking-[0.2em]">
              {{ t('auth.confirmEmail.loading') }}
            </p>
          </div>

          <div v-else class="w-full space-y-6 flex flex-col items-center">
            <p :class="[
              'text-sm leading-relaxed max-w-[280px]',
              success ? 'text-zinc-400' : 'text-red-400 font-medium'
            ]">
              {{ message }}
            </p>

            <div class="w-full pt-2">
              <router-link v-if="success" to="/login" class="w-full">
                <Button class="w-full bg-blue-600 hover:bg-blue-500 text-white rounded-xl h-12 font-black uppercase tracking-widest transition-all hover:gap-4 flex items-center justify-center gap-2">
                  {{ t('auth.confirmEmail.goLogin') }}
                  <ArrowRight class="w-4 h-4" />
                </Button>
              </router-link>

              <router-link v-else to="/register" class="w-full">
                <Button variant="outline" class="w-full border-zinc-800 text-zinc-400 hover:text-white hover:bg-zinc-800 rounded-xl h-12 font-black uppercase tracking-widest transition-all">
                  Back to Register
                </Button>
              </router-link>
            </div>
          </div>
        </div>

        <div class="mt-10 pt-8 border-t border-zinc-800/50 w-full flex justify-center">
          <p class="text-[10px] text-zinc-600 uppercase font-black tracking-[0.3em]">
            HomeRecipes Security
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.space-y-6 {
  animation: slideUp 0.5s cubic-bezier(0.16, 1, 0.3, 1) forwards;
}

@keyframes slideUp {
  from { opacity: 0; transform: translateY(12px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>