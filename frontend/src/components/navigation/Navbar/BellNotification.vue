<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { useWebSocket } from '@vueuse/core'
import { Bell, ExternalLink, Heart, Info, MessageSquare } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover'
import {
  getNotifications,
  getNotificationWsUrl,
  markAllNotificationsAsRead,
  markNotificationAsRead
} from '@/api/notification'
import { type Notification, NotificationCategory } from '@/models/Notification'
import { formatDistanceToNow } from 'date-fns'
import { ROUTES } from "@/router/routes.ts"

const notifications = ref<Notification[]>([])
const unreadCount = ref(0)

// 1. Load initial data
const fetchItems = async () => {
  try {
    const res = await getNotifications()
    notifications.value = res.items
    unreadCount.value = res.unread_count
  } catch (err) {
    console.error("Failed to load notifications", err)
  }
}
fetchItems()

// 2. Setup VueUse WebSocket
const { data } = useWebSocket(getNotificationWsUrl(), {
  heartbeat: { message: 'ping', interval: 30000 },
  autoReconnect: true
})

// 3. Reactively handle new messages
watch(data, (newData) => {
  if (newData) {
    try {
      const newNotif: Notification = JSON.parse(newData)
      notifications.value = [newNotif, ...notifications.value]
      unreadCount.value++
    } catch (e) {
      // Handle heartbeats or malformed JSON
    }
  }
})

const handleMarkAll = async () => {
  await markAllNotificationsAsRead()
  notifications.value.forEach(n => n.is_read = true)
  unreadCount.value = 0
}

const handleNotificationClick = async (n: Notification) => {
  if (!n.is_read) {
    await markNotificationAsRead(n.id)
    n.is_read = true
    unreadCount.value = Math.max(0, unreadCount.value - 1)
  }
}

const getFormattedMessageParts = (notification: Notification) => {
  const { message, variables: vars } = notification
  const placeholderRegex = /({actor}|{acteur}|VAR_?A|{recipe_title}|{recette title}|{recipe title}|VAR_?R)/gi

  if (!message || !placeholderRegex.test(message)) {
    return [{ text: message, isLink: false, route: null }]
  }

  const parts = message.split(placeholderRegex)

  return parts.map(part => {
    const isPlaceholder = part.match(placeholderRegex)

    if (isPlaceholder) {
      const isActor = /actor|acteur|VAR_?A/i.test(part)
      const lookupKey = isActor ? 'actor' : 'recipe_title'

      return {
        text: vars[lookupKey] || part,
        isLink: true,
        route: isActor
            ? ROUTES.USER.PROFILE(notification.actor_id!)
            : ROUTES.RECIPE(notification.target_id!)
      }
    }

    return { text: part, isLink: false, route: null }
  })
}

const getIcon = (category: string) => {
  switch (category) {
    case NotificationCategory.RecipeFavorite: return Heart
    case NotificationCategory.RecipeComment: return MessageSquare
    case NotificationCategory.CommentReply: return MessageSquare
    default: return Info
  }
}
</script>
<template>
  <Popover>
    <PopoverTrigger as-child>
      <Button variant="ghost" size="icon" class="relative group">
        <Bell class="h-5 w-5 text-muted-foreground group-hover:text-primary transition-colors"/>
        <span v-if="unreadCount > 0"
              class="absolute top-1.5 right-1.5 flex h-3.5 w-3.5 items-center justify-center rounded-full bg-destructive text-[9px] text-white font-black border-2 border-background animate-in zoom-in">
          {{ unreadCount }}
        </span>
      </Button>
    </PopoverTrigger>

    <PopoverContent
        class="w-[94vw] sm:w-120 p-0 rounded-3xl overflow-hidden shadow-2xl mt-2 border border-border bg-white dark:bg-[#0f0f0f] dark:border-neutral-800"
        align="end"
        :side-offset="8"
    >
      <div class="p-4 border-b border-border bg-neutral-50 dark:bg-neutral-900/50 flex justify-between items-center">
        <h3 class="font-bold text-[10px] uppercase tracking-widest text-neutral-500 dark:text-neutral-400">Notifications</h3>
        <button @click="handleMarkAll"
                class="text-[10px] text-primary font-bold uppercase hover:opacity-80 transition-opacity">
          Mark all read
        </button>
      </div>

      <div class="max-h-[70vh] sm:max-h-100 overflow-y-auto custom-scrollbar bg-white dark:bg-[#0f0f0f]">
        <div v-if="notifications.length === 0" class="p-12 text-center text-neutral-400 dark:text-neutral-600">
          <Bell class="h-8 w-8 mx-auto mb-3 opacity-20" />
          <p class="text-xs">No new updates yet.</p>
        </div>

        <div v-for="n in notifications" :key="n.id"
             @click="handleNotificationClick(n)"
             :class="[
               'p-4 border-b border-border last:border-0 cursor-pointer transition-all duration-200',
               !n.is_read ? 'bg-primary/[0.04] dark:bg-primary/[0.08]' : 'hover:bg-neutral-50 dark:hover:bg-neutral-900/40'
             ]">
          <div class="flex gap-4">
            <div :class="[
              'mt-1 flex h-10 w-10 items-center justify-center rounded-2xl shrink-0 border transition-all',
              !n.is_read
                ? 'bg-primary/10 text-primary border-primary/20 shadow-[0_0_15px_rgba(var(--primary),0.1)]'
                : 'bg-neutral-100 dark:bg-neutral-800 text-neutral-500 border-transparent'
            ]">
              <component :is="getIcon(n.category)" class="h-5 w-5"/>
            </div>

            <div class="flex-1 min-w-0">
              <div class="flex justify-between items-start mb-1">
                <p class="text-sm font-bold leading-none text-neutral-900 dark:text-neutral-100">{{ n.title }}</p>
                <div v-if="!n.is_read" class="h-2 w-2 rounded-full bg-primary shrink-0 ml-2" />
              </div>

              <div class="text-xs text-neutral-600 dark:text-neutral-400 leading-relaxed">
                <template v-for="(part, idx) in getFormattedMessageParts(n)" :key="idx">
                  <router-link
                      v-if="part.isLink"
                      :to="part.route"
                      @click.stop
                      class="font-extrabold text-primary hover:underline"
                  >
                    {{ part.text }}
                  </router-link>

                  <span v-else :class="[
                    part.text.startsWith(': ')
                      ? 'block mt-2 pl-3 border-l-2 border-primary/40 italic text-neutral-800 dark:text-neutral-200 bg-neutral-100/50 dark:bg-neutral-800/50 py-2 rounded-r-xl'
                      : ''
                  ]">
                    {{ part.text.startsWith(': ') ? part.text.substring(2) : part.text }}
                  </span>
                </template>
              </div>

              <div class="flex items-center justify-between mt-4">
                <p class="text-[10px] text-neutral-400 font-bold uppercase tracking-tighter">
                  {{ formatDistanceToNow(new Date(n.created_at)) }} ago
                </p>

                <router-link
                    v-if="n.target_id"
                    :to="ROUTES.RECIPE(n.target_id)"
                    @click.stop
                    class="text-[10px] font-black text-primary uppercase flex items-center gap-1"
                >
                  View Detail
                  <ExternalLink class="h-3 w-3"/>
                </router-link>
              </div>
            </div>
          </div>
        </div>
      </div>
    </PopoverContent>
  </Popover>
</template>

<style scoped>
/* Dark Mode Scrollbar Fix */
.custom-scrollbar::-webkit-scrollbar {
  width: 5px;
}

.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}

.custom-scrollbar::-webkit-scrollbar-thumb {
  /* Using hex for guaranteed contrast in dark mode */
  background: #333;
  border-radius: 10px;
}

.dark .custom-scrollbar::-webkit-scrollbar-thumb {
  background: #444; /* Slightly lighter in dark mode for visibility */
}

.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: hsl(var(--primary));
}

/* Firefox */
.custom-scrollbar {
  scrollbar-width: thin;
  scrollbar-color: #444 transparent;
}
</style>