import type { User } from '@/models/User'

export function getInitials(user: User | null | undefined): string {
    if (!user) return ''

    const first = user.first_name?.trim()
    const last = user.last_name?.trim()

    if (!first && !last) return ''

    if (first && !last) return first.charAt(0).toUpperCase()
    if (!first && last) return last.charAt(0).toUpperCase()

    return `${first!.charAt(0)}${last!.charAt(0)}`.toUpperCase()
}

export function useInitials() {
    return { getInitials }
}
