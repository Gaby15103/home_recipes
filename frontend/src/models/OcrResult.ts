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
    actions: LexiconMatch[]; // Changed from string[] to LexiconMatch[] to get bilingual terms
    original_line: string;
    // New Bilingual Display Names
    display_name_en: string;
    display_name_fr: string;
    position: number;
}

export interface OcrIngredientGroup {
    // New Bilingual Names
    name_en: string;
    name_fr: string;
    ingredients: OcrIngredient[];
}

export interface OcrStep {
    position: number;
    // New Bilingual Steps
    raw_text_en: string;
    raw_text_fr: string;
    detected_actions: LexiconMatch[];
    detected_equipment: LexiconMatch[];
}

export interface OcrStepGroup {
    // New Bilingual Names
    name_en: string;
    name_fr: string;
    steps: OcrStep[];
}

export interface OcrRecipeResponse {
    primary_language: string;
    // New Bilingual Titles
    title_en: string;
    title_fr: string;
    detected_servings: number | null;
    ingredient_groups: OcrIngredientGroup[];
    step_groups: OcrStepGroup[];
    unparsed_segments: string[];
    raw_text: string;
}