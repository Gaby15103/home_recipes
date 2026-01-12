import type { LucideIcon } from 'lucide-vue-next';

export interface BreadcrumbItem {
    title: string;
    href: string;
}

export interface NavItem {
    title: string;
    href: string;
    icon?: LucideIcon;
    isActive?: boolean;
    subNavItems?: SubNavItem[];
}
export interface SubNavItem {
    title: string;
    href: string;
    icon?: LucideIcon;
}



export type BreadcrumbItemType = BreadcrumbItem;