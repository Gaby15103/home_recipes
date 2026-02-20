import {api} from "@/api/client.ts";
import {UnitRoutes} from "@/api/routes.ts";
import type {Unit} from "@/models/Recipe"
import type {UnitInput} from "@/models/RecipeCreate.ts";

export function getUnits() {
    return api<Unit[]>(UnitRoutes.list(), { method: "GET" });
}
export function getUnit(unit_id:string) {
    return api<Unit>(UnitRoutes.get(unit_id), { method: "GET" });
}
export function createUnit(new_unit:UnitInput) {
    return api<Unit>(UnitRoutes.create(), { method: "POST", data: new_unit });
}
export function updateUnit(unit_id:string, updated_unit:Unit) {
    return api<Unit>(UnitRoutes.update(unit_id), { method: "PUT", data:updated_unit  });
}