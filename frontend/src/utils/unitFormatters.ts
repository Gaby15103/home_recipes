import type {Unit} from "@/models/Recipe.ts";

export function formatQuantity(value: number, isPiece: boolean): string {
    if (value === 0) return "";

    // Handle "Number" units: We don't want 1.00 garlic bulbs
    if (isPiece) return Math.round(value).toString();

    const tolerance = 0.03;
    const whole = Math.floor(value);
    const fraction = value - whole;

    const commonFractions: [number, string][] = [
        [0.125, "1/8"], [0.25, "1/4"], [0.333, "1/3"],
        [0.5, "1/2"], [0.666, "2/3"], [0.75, "3/4"]
    ];

    const match = commonFractions.find(([f]) => Math.abs(fraction - f) < tolerance);

    if (match) {
        return whole > 0 ? `${whole} ${match[1]}` : match[1];
    }

    return value % 1 === 0 ? value.toString() : value.toFixed(2);
}

export function convertUnits(
    value: number,
    sourceUnit: Unit,
    targetUnit: Unit): number {
    if (sourceUnit.id === targetUnit.id) return value;

    const baseValue = value * (sourceUnit.conversion_factor || 1);

    const converted = baseValue / (targetUnit.conversion_factor || 1);

    return Number(converted.toFixed(2));
}