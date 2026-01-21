import { computed, readonly } from "vue";
import { useRoute } from "vue-router";
import { toUrl } from "@/lib/utils";

export function useActiveUrl() {
    const route = useRoute();

    const currentUrl = computed(() => {
        return route.path;
    });

    function urlIsActive(urlToCheck: string) {
        return toUrl(urlToCheck) === currentUrl.value;
    }

    return {
        currentUrl: readonly(currentUrl),
        urlIsActive,
    };
}
