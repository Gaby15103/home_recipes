import type { ClassValue } from "clsx"
import { clsx } from "clsx"
import { twMerge } from "tailwind-merge"
import type {Ref} from "vue";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
export type Href = string | { url: string }

export function toUrl(href: Href) {
  return typeof href === 'string' ? href : href.url
}
export function valueUpdater<T>(updaterOrValue: T | ((oldValue: T) => T), refValue: Ref<T>) {
  if (typeof updaterOrValue === 'function') {
    // If it's a function, call it with the old value
    refValue.value = (updaterOrValue as (old: T) => T)(refValue.value)
  } else {
    // Otherwise, just assign the value
    refValue.value = updaterOrValue
  }
}
