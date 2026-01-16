import { api } from "./client";
import { TagRoutes } from "./routes";
import type { Tag, InputTag } from "@/models/Tag";

export function getTags() {
    return api<Tag[]>(TagRoutes.list(), { method: "GET" });
}

export function createTag(tag: InputTag) {
    return api<Tag>(TagRoutes.create(), { method: "POST", data: tag });
}

export function updateTag(id: string, tag: Partial<InputTag>) {
    return api<Tag>(TagRoutes.update(id), { method: "PUT", data: tag });
}
