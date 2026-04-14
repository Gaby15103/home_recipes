<script setup lang="ts">
import {ref, watch} from 'vue'
import {useRouter} from 'vue-router'
import {useWebSocket} from '@vueuse/core' // Refactored to use VueUse
import {Bell, ExternalLink, Heart, Info, MessageSquare} from 'lucide-vue-next'
import {Button} from '@/components/ui/button'
import {Popover, PopoverContent, PopoverTrigger} from '@/components/ui/popover'
import {
  getNotifications,
  getNotificationWsUrl,
  markAllNotificationsAsRead,
  markNotificationAsRead
} from '@/api/notification'
import {type Notification, NotificationCategory} from '@/models/Notification'
import {formatDistanceToNow} from 'date-fns'
import {ROUTES} from "@/router/routes.ts"

const router = useRouter()
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
// This handles the connection, auto-reconnect, and heartbeat automatically.
const { data } = useWebSocket(getNotificationWsUrl(), {
  onConnected(ws) {
    console.log('Connected!')
  },
  onDisconnected(ws, event) {
    console.log('Disconnected!', event.code)
  },
  onError(ws, event) {
    console.error('Error:', event)
  },
  onMessage(ws, event) {
    console.log('Message:', event.data)
  },
})

// 3. Reactively handle new messages
watch(data, (newData) => {
  if (newData) {
    try {
      const newNotif: Notification = JSON.parse(newData)
      // Instant update for the red dot and list
      notifications.value = [newNotif, ...notifications.value]
      unreadCount.value++
    } catch (e) {
      // Ignore heartbeat pongs from server
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

  const isRecipeAction = [
    NotificationCategory.RecipeFavorite,
    NotificationCategory.RecipeComment
  ].includes(n.category as NotificationCategory)
}

const getFormattedMessageParts = (message: string) => {
  const placeholderRegex = /({actor}|{author}|{acteur})/g
  if (!placeholderRegex.test(message)) return [{ text: message, isLink: false }]

  const parts = message.split(placeholderRegex)
  return parts.map(part => ({
    text: part,
    isLink: placeholderRegex.test(part)
  }))
}

const getIcon = (category: string) => {
  switch (category) {
    case NotificationCategory.RecipeFavorite: return Heart
    case NotificationCategory.RecipeComment: return MessageSquare
    default: return Info
  }
}
</script>

<template>
  <Popover>
    <PopoverTrigger as-child>
      <Button variant="ghost" size="icon" class="relative">
        <Bell class="h-5 w-5" />
        <span v-if="unreadCount > 0"
              class="absolute top-1 right-1 flex h-4 w-4 items-center justify-center rounded-full bg-destructive text-[10px] text-white font-bold border-2 border-background animate-in zoom-in">
          {{ unreadCount }}
        </span>
      </Button>
    </PopoverTrigger>

    <PopoverContent class="w-80 p-0 rounded-2xl overflow-hidden shadow-xl" align="end">
      <div class="p-4 border-b bg-secondary/10 flex justify-between items-center">
        <h3 class="font-bold text-xs uppercase tracking-widest text-foreground/70">Notifications</h3>
        <button @click="handleMarkAll"
                class="text-[10px] text-primary font-bold uppercase hover:underline">
          Mark all read
        </button>
      </div>

      <div class="max-h-100 overflow-y-auto">
        <div v-if="notifications.length === 0" class="p-8 text-center text-muted-foreground text-sm">
          No new updates.
        </div>

        <div v-for="n in notifications" :key="n.id"
             @click="handleNotificationClick(n)"
             :class="['p-4 border-b last:border-0 cursor-pointer transition-colors hover:bg-secondary/5', !n.is_read ? 'bg-primary/5' : '']">
          <div class="flex gap-3">
            <div :class="['mt-1 flex h-8 w-8 items-center justify-center rounded-full shrink-0', !n.is_read ? 'bg-primary/20 text-primary' : 'bg-muted text-muted-foreground']">
              <component :is="getIcon(n.category)" class="h-4 w-4" />
            </div>

            <div class="flex-1 min-w-0">
              <p class="text-sm font-semibold leading-none mb-1 truncate">{{ n.title }}</p>

              <p class="text-xs text-muted-foreground leading-relaxed">
                <template v-for="(part, idx) in getFormattedMessageParts(n.message)" :key="idx">
                  <router-link
                      v-if="part.isLink && n.actor_id"
                      :to="ROUTES.USER.PROFILE(n.actor_id)"
                      @click.stop
                      class="font-bold text-primary hover:underline"
                  >
                    {{ n.actor_name || 'Someone' }}
                  </router-link>
                  <span v-else>{{ part.text }}</span>
                </template>
              </p>

              <div class="flex items-center justify-between mt-3">
                <p class="text-[10px] text-muted-foreground/60 font-medium uppercase">
                  {{ formatDistanceToNow(new Date(n.created_at)) }} ago
                </p>

                <router-link
                    v-if="n.target_id"
                    :to="ROUTES.RECIPE(n.target_id)"
                    @click.stop
                    class="text-[10px] font-bold text-primary uppercase hover:underline flex items-center gap-1"
                >
                  View Recipe
                  <ExternalLink class="h-2.5 w-2.5" />
                </router-link>
              </div>
            </div>
          </div>
        </div>
      </div>
    </PopoverContent>
  </Popover>
</template>