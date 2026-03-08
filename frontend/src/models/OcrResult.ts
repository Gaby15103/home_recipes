// models/OcrResult.ts

export interface LexiconMatch {
    raw_token: string;
    lexicon_id: number;
    term_en: string;
    term_fr: string;
    category: "ingredient" | "unit" | "descriptor" | "action";
    confidence: number;
    match_strategy: string;
}

export interface OcrIngredient {
    quantity: number | null;
    unit: LexiconMatch | null;
    ingredient: LexiconMatch | null;
    actions: string[];
    original_line: string;
    position: number;
}

export interface OcrIngredientGroup {
    name: string;
    ingredients: OcrIngredient[];
}

export interface OcrStep {
    position: number;
    raw_text: string;
    detected_actions: string[];
    detected_equipment: string[];
}

export interface OcrStepGroup {
    name: string;
    steps: OcrStep[];
}

export interface OcrRecipeResponse {
    primary_language: string;
    title: string;
    detected_servings: number;
    ingredient_groups: OcrIngredientGroup[];
    step_groups: OcrStepGroup[];
    unparsed_segments: string[];
    raw_text: string;
}