import { computed, readonly } from 'vue'
import { toUrl } from '@/lib/utils'

const currentUrlReactive = computed(() => {
    if (typeof window === 'undefined') return '/'
    return new URL(window.location.href).pathname
})

export function useActiveUrl() {
    function urlIsActive(
        urlToCheck: string,
        currentUrl?: string,
    ) {
        const urlToCompare = currentUrl ?? currentUrlReactive.value
        return toUrl(urlToCheck) === urlToCompare
    }

    return {
        currentUrl: readonly(currentUrlReactive),
        urlIsActive,
    }
}
